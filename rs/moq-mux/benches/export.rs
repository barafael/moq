//! Export and ordered startup cost swept over independent track/group and frame counts.
use std::time::Duration;

use bytes::Bytes;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use hang::catalog::{AAC, AudioConfig};
use moq_mux::catalog::hang::Container;
use moq_mux::container::{Container as _, Frame, Kind};
use moq_net::Timestamp;

/// What keeps a prepared broadcast served; dropping it stops the origin's tasks.
struct Fixture {
	/// The broadcast, its catalog and its tracks, which a reader needs kept open.
	_published: Box<dyn std::any::Any>,
	_origin: moq_net::origin::Producer,
	driver: tokio::task::JoinHandle<moq_net::Error>,
	requests: tokio::task::JoinHandle<()>,
}

impl Drop for Fixture {
	fn drop(&mut self) {
		self.driver.abort();
		self.requests.abort();
	}
}

/// A finished broadcast of `tracks` audio tracks with `frames` queued frames each.
async fn prepare(tracks: usize, frames: usize) -> (moq_mux::Source, moq_mux::catalog::Consumer, Fixture) {
	let mut broadcast = moq_net::broadcast::Info::new().produce();
	let consumer = broadcast.consume();
	let mut catalog = moq_mux::catalog::Producer::new(&mut broadcast, Default::default()).unwrap();
	let stream = moq_mux::catalog::Consumer::Hang(catalog.consume().unwrap());
	let mut publishers = Vec::new();
	for index in 0..tracks {
		let track = broadcast
			.create_track(
				format!("audio{index}"),
				catalog.track_info(hang::catalog::PRIORITY.audio),
			)
			.unwrap();
		let mut config = AudioConfig::new(AAC { profile: 2 }, 44100, 2);
		config.description = Some(Bytes::from_static(&[0x12, 0x10]));
		publishers.push(catalog.audio(track, Container::Legacy(Kind::Audio), config).unwrap());
	}
	for track in &mut publishers {
		for index in 0..frames {
			track
				.write(Frame {
					timestamp: Timestamp::from_millis(index as u64 * 20).unwrap(),
					duration: None,
					payload: Bytes::from_static(b"audio-frame"),
					keyframe: index == 0,
				})
				.unwrap();
		}
		track.finish().unwrap();
	}
	catalog.finish().unwrap();
	let (origin, driver) = moq_net::origin::Producer::new(Default::default());
	let driver = tokio::spawn(moq_net::time::run(driver));
	let dynamic = origin.dynamic("", Default::default()).unwrap();
	let requests = tokio::spawn(async move {
		while let Ok(request) = dynamic.requested_broadcast().await {
			request.accept(consumer.clone());
		}
	});
	let resolved = origin.consume().request_broadcast("bench", None).await.unwrap();
	let source = moq_mux::Source::new(origin.consume(), resolved);
	let fixture = Fixture {
		_published: Box::new((broadcast, catalog, publishers)),
		_origin: origin,
		driver,
		requests,
	};
	(source, stream, fixture)
}

async fn export(format: &str, source: moq_mux::Source, stream: moq_mux::catalog::Consumer) -> usize {
	let mut chunks = 0;
	if format == "flv" {
		let mut export = moq_mux::container::flv::Export::new(source, stream).with_multitrack(true);
		while let Some(chunk) = export.next().await.unwrap() {
			std::hint::black_box(chunk);
			chunks += 1;
		}
	} else {
		let mut export = moq_mux::container::mkv::Export::new(source, stream).with_fragment_duration(Duration::ZERO);
		while let Some(chunk) = export.next().await.unwrap() {
			std::hint::black_box(chunk);
			chunks += 1;
		}
	}
	chunks
}

fn bench_export(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();
	let mut group = c.benchmark_group("container-export");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(1));
	for format in ["flv", "mkv"] {
		for tracks in [1, 4, 16] {
			for frames in [1, 16, 64] {
				group.throughput(Throughput::Elements((tracks * frames) as u64));
				group.bench_function(BenchmarkId::new(format, format!("{tracks}x{frames}")), |b| {
					// Publishing the broadcast is setup; only exporting it is measured.
					b.iter_batched(
						|| runtime.block_on(prepare(tracks, frames)),
						|(source, stream, fixture)| {
							let chunks = runtime.block_on(export(format, source, stream));
							assert_eq!(chunks, 1 + tracks * frames);
							fixture
						},
						BatchSize::PerIteration,
					);
				});
			}
		}
	}
	group.finish();
}

fn bench_startup(c: &mut Criterion) {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();
	let _entered = runtime.enter();
	let mut group = c.benchmark_group("container-startup");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(1));
	for groups in [1usize, 16, 256] {
		for frames in [1usize, 4, 16] {
			group.throughput(Throughput::Elements((groups * frames) as u64));
			group.bench_function(BenchmarkId::new("first-read", format!("{groups}x{frames}")), |b| {
				b.iter_batched(
					|| {
						let track = moq_net::broadcast::Info::new()
							.produce()
							.create_track("startup", hang::container::track_info(hang::catalog::PRIORITY.video))
							.unwrap();
						let budget = Duration::from_micros((groups * frames - 1) as u64);
						let subscriber =
							track.subscribe(moq_net::track::Subscription::default().with_max_delay(budget));
						let format = Container::Legacy(Kind::Data);
						// Reverse arrival order, like a retained window sent newest-first.
						for sequence in (0..groups).rev() {
							let mut group = track
								.create_group(moq_net::group::Info {
									sequence: sequence as u64,
								})
								.unwrap();
							for index in 0..frames {
								format
									.write(
										&mut group,
										&[Frame {
											timestamp: Timestamp::from_micros((sequence * frames + index) as u64)
												.unwrap(),
											payload: Bytes::from_static(&[1]),
											keyframe: index == 0,
											duration: None,
										}],
									)
									.unwrap();
							}
							group.finish().unwrap();
						}
						(moq_mux::container::Consumer::new(subscriber, format), track)
					},
					|(mut consumer, track)| {
						let waiter = kio::Waiter::new(std::task::Waker::noop().clone());
						assert!(matches!(
							consumer.poll_read(&waiter),
							std::task::Poll::Ready(Ok(Some(moq_mux::container::Read::Frame { group: 0, .. })))
						));
						(consumer, track)
					},
					BatchSize::PerIteration,
				);
			});
		}
	}
	group.finish();
}

criterion_group!(benches, bench_export, bench_startup);
criterion_main!(benches);
