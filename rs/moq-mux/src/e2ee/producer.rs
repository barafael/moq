use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::task::{Poll, ready};

use bytes::Bytes;
use moq_e2ee::{Name, generation};

use crate::catalog::hang::{Catalog, Extra};

/// Encrypts a plaintext hang broadcast into a protected one.
///
/// Reads `source`, a local broadcast that is never announced, and writes each track its catalog
/// names to `output` under its physical name. Announce `output` at
/// [`Credential::path`](moq_e2ee::Credential::path) with
/// [`generation.epoch()`](moq_e2ee::generation::Producer::epoch) on its route. See the
/// [module docs](super) for what is carried.
///
/// Every group is carried, whatever its age, so a timeline over the output resolves. A track
/// that ends is finished on the output too, and a track the catalog later names again under the
/// same name is not carried: its physical name is spent for this generation. Dropping the
/// producer before [`run`](Self::run) returns aborts the output tracks.
pub struct Producer {
	source: moq_net::broadcast::Consumer,
	generation: generation::Producer,
	output: moq_net::broadcast::Producer,
	catalog: CatalogTrack,
	/// Every plaintext track the catalog has named. Each is carried at most once.
	named: BTreeSet<String>,
	/// Named tracks whose plaintext subscription has not resolved yet, keyed by plaintext name.
	subscribing: BTreeMap<String, Subscribing>,
	/// The tracks being carried, each re-polled only when it wakes.
	forwards: kio::Tasks<Forward>,
	/// The first encryption error a carried track hit.
	failed: Failed,
}

/// Where a carried track leaves the error that ends the whole broadcast.
type Failed = Arc<Mutex<Option<crate::Error>>>;

impl Producer {
	/// Encrypt `source` into `output` under `generation`.
	///
	/// Creates the physical catalog tracks on `output` now, so announce it afterwards.
	///
	/// # Errors
	///
	/// A net error if `output` already carries a physical catalog name or `source` has ended.
	pub fn new(
		source: moq_net::broadcast::Consumer,
		generation: generation::Producer,
		output: moq_net::broadcast::Producer,
	) -> crate::Result<Self> {
		let catalog_track = |semantic: &str| -> crate::Result<moq_e2ee::track::Producer> {
			let name = generation.name(semantic)?;
			let track = output.create_track(name.as_str(), hang::Catalog::default_track_info())?;
			Ok(generation.produce(track)?)
		};
		let hang = catalog_track(hang::Catalog::DEFAULT_NAME)?;
		let hangz = catalog_track(hang::Catalog::COMPRESSED_NAME)?;
		let subscribing = source.track(hang::Catalog::DEFAULT_NAME)?.subscribe(subscription());

		Ok(Self {
			source,
			generation,
			output,
			catalog: CatalogTrack {
				source: Source::Subscribing(subscribing),
				group: None,
				snapshot: None,
				staged: None,
				hang: Some(hang),
				hangz: Some(hangz),
			},
			named: BTreeSet::new(),
			subscribing: BTreeMap::new(),
			forwards: kio::Tasks::new(),
			failed: Failed::default(),
		})
	}

	/// Carry the broadcast until its catalog and every track it named have ended.
	///
	/// # Errors
	///
	/// A catalog that cannot be read or protected (see the [module docs](super)), the source
	/// catalog track failing, or an encryption error on any track. The output tracks are aborted.
	pub fn poll_run(&mut self, waiter: &kio::Waiter) -> Poll<crate::Result<()>> {
		let res = self.poll_carry(waiter);
		if let Poll::Ready(Err(err)) = &res {
			tracing::warn!(%err, "aborting the encrypted broadcast");
			self.abort();
		}
		res
	}

	/// Carry the broadcast until its catalog and every track it named have ended.
	///
	/// # Errors
	///
	/// Same as [`Self::poll_run`].
	pub async fn run(mut self) -> crate::Result<()> {
		kio::wait(|waiter| self.poll_run(waiter)).await
	}

	fn poll_carry(&mut self, waiter: &kio::Waiter) -> Poll<crate::Result<()>> {
		let catalog = self.poll_catalog(waiter)?;
		self.poll_subscribing(waiter)?;
		let forwards = self.forwards.poll(waiter);
		if let Some(err) = self.failed.lock().expect("failed").take() {
			return Poll::Ready(Err(err));
		}

		self.emit()?;

		if catalog.is_pending()
			|| forwards.is_pending()
			|| !self.subscribing.is_empty()
			|| self.catalog.staged.is_some()
		{
			return Poll::Pending;
		}
		for track in [self.catalog.hang.take(), self.catalog.hangz.take()]
			.into_iter()
			.flatten()
		{
			track.finish()?;
		}
		Poll::Ready(Ok(()))
	}

	/// Read the newest plaintext catalog and stage its protected form, ready once the source ends.
	fn poll_catalog(&mut self, waiter: &kio::Waiter) -> Poll<crate::Result<()>> {
		loop {
			let source = match &mut self.catalog.source {
				Source::Subscribing(subscribing) => {
					let subscriber = ready!(subscribing.poll_ok(waiter))?;
					self.catalog.source = Source::Live(subscriber.ordered());
					continue;
				}
				Source::Live(source) => source,
				Source::Ended => break,
			};
			match source.poll_next_group(waiter)? {
				// A newer snapshot supersedes one still being read.
				Poll::Ready(Some(group)) => self.catalog.group = Some(group),
				Poll::Ready(None) => self.catalog.source = Source::Ended,
				Poll::Pending => break,
			}
		}

		if let Some(group) = &mut self.catalog.group
			&& let Poll::Ready(frame) = group.poll_read_frame(waiter)
		{
			let group = self.catalog.group.take().expect("polled above");
			match frame {
				Ok(Some(frame)) => {
					self.stage(group.sequence, frame)?;
					self.catalog.snapshot = Some(group);
				}
				// An empty group carries no catalog, and a failed one leaves the last one standing.
				Ok(None) => {}
				Err(err) => tracing::debug!(sequence = group.sequence, %err, "the plaintext catalog group failed"),
			}
		}

		// Deltas would need re-encoding against the protected snapshot; the catalog producer never
		// writes them, so one here is refused rather than dropped.
		if let Some(snapshot) = &mut self.catalog.snapshot
			&& let Poll::Ready(frame) = snapshot.poll_read_frame(waiter)
		{
			self.catalog.snapshot = None;
			if let Ok(Some(_)) = frame {
				return Poll::Ready(Err(crate::Error::Unencryptable("a catalog delta")));
			}
		}

		match self.catalog.source {
			Source::Ended if self.catalog.group.is_none() => Poll::Ready(Ok(())),
			_ => Poll::Pending,
		}
	}

	/// Protect one plaintext catalog and start carrying every track it newly names.
	fn stage(&mut self, sequence: u64, frame: moq_net::frame::Frame) -> crate::Result<()> {
		let mut catalog = Catalog::<Extra>::from_slice(&frame.payload)?;
		let tracks = super::catalog::protect(&mut catalog, &self.generation)?;
		let json = Bytes::from(serde_json::to_vec(&catalog).map_err(hang::Error::from)?);

		for (name, physical) in &tracks {
			if !self.named.insert(name.clone()) {
				continue;
			}
			match self.source.track(name) {
				Ok(track) => {
					let pending = track.subscribe(subscription());
					let physical = *physical;
					self.subscribing.insert(name.clone(), Subscribing { physical, pending });
				}
				Err(err) => {
					tracing::warn!(track = %name, %err, "the catalog names a track the broadcast does not carry")
				}
			}
		}

		self.catalog.staged = Some(Staged {
			sequence,
			timestamp: frame.timestamp,
			json,
			tracks: tracks.into_keys().collect(),
		});
		Ok(())
	}

	/// Create the output track for each plaintext subscription that resolved, and start carrying it.
	fn poll_subscribing(&mut self, waiter: &kio::Waiter) -> crate::Result<()> {
		let mut resolved = Vec::new();
		for (name, subscribing) in &self.subscribing {
			if let Poll::Ready(res) = subscribing.pending.poll_ok(waiter) {
				resolved.push((name.clone(), subscribing.physical, res));
			}
		}
		for (name, physical, res) in resolved {
			self.subscribing.remove(&name);
			let subscriber = match res {
				Ok(subscriber) => subscriber,
				Err(err) => {
					tracing::warn!(track = %name, %err, "the catalog names a track that ended before it was carried");
					continue;
				}
			};
			let track = self.output.create_track(physical.as_str(), subscriber.info().clone())?;
			self.forwards.push(Forward {
				name,
				source: subscriber.ordered(),
				output: Some(self.generation.produce(track)?),
				groups: Vec::new(),
				ended: false,
				failed: self.failed.clone(),
			});
		}
		Ok(())
	}

	/// Publish the staged catalog once every track it names exists on the output.
	///
	/// A subscriber reads a physical name from the catalog and subscribes to it straight away, and
	/// the output has no handler to wait on a track that does not exist yet.
	fn emit(&mut self) -> crate::Result<()> {
		let Some(staged) = &self.catalog.staged else {
			return Ok(());
		};
		if staged.tracks.iter().any(|name| self.subscribing.contains_key(name)) {
			return Ok(());
		}
		let staged = self.catalog.staged.take().expect("checked above");
		let (Some(hang), Some(hangz)) = (&mut self.catalog.hang, &mut self.catalog.hangz) else {
			return Ok(());
		};

		// Compressed before it is encrypted: ciphertext does not compress.
		let compressed = moq_flate::Encoder::new().frame(&staged.json);
		for (track, payload) in [(hang, &staged.json), (hangz, &compressed)] {
			let mut group = track.create_group(staged.sequence)?;
			group.write_frame(staged.timestamp, payload)?;
			group.finish()?;
		}
		Ok(())
	}

	/// Abort every output track that is still open.
	fn abort(&mut self) {
		for track in [self.catalog.hang.take(), self.catalog.hangz.take()]
			.into_iter()
			.flatten()
		{
			let _ = track.abort();
		}
		self.subscribing.clear();
		// Each carried track aborts its output as it drops.
		self.forwards = kio::Tasks::new();
	}
}

impl Drop for Producer {
	fn drop(&mut self) {
		self.abort();
	}
}

/// Every group a local track still holds, so none is skipped before it is carried.
fn subscription() -> moq_net::track::Subscription {
	moq_net::track::Subscription::default().with_max_delay(std::time::Duration::MAX)
}

struct CatalogTrack {
	source: Source,
	/// The newest plaintext catalog group, before its snapshot is read.
	group: Option<moq_net::group::Consumer>,
	/// The group whose snapshot was staged, watched only to refuse a delta.
	snapshot: Option<moq_net::group::Consumer>,
	/// A protected catalog waiting for the tracks it names to exist on the output.
	staged: Option<Staged>,
	/// The physical `catalog.json`, until finished or aborted.
	hang: Option<moq_e2ee::track::Producer>,
	/// The physical `catalog.json.z`, until finished or aborted.
	hangz: Option<moq_e2ee::track::Producer>,
}

enum Source {
	Subscribing(kio::Pending<moq_net::track::Subscribing>),
	Live(moq_net::track::Ordered),
	Ended,
}

struct Staged {
	sequence: u64,
	timestamp: Option<moq_net::Timestamp>,
	json: Bytes,
	/// The plaintext names of the tracks this catalog carries.
	tracks: Vec<String>,
}

struct Subscribing {
	physical: Name,
	pending: kio::Pending<moq_net::track::Subscribing>,
}

/// Copies one plaintext track onto its protected output, keeping every sequence and frame index.
///
/// Aborts its output if dropped before the source ends.
struct Forward {
	/// The plaintext name, for logs.
	name: String,
	source: moq_net::track::Ordered,
	/// Taken when the track finishes or aborts.
	output: Option<moq_e2ee::track::Producer>,
	groups: Vec<Group>,
	ended: bool,
	failed: Failed,
}

struct Group {
	source: moq_net::group::Consumer,
	output: moq_e2ee::group::Producer,
}

impl kio::Task for Forward {
	type Output = ();

	fn poll(&mut self, waiter: &kio::Waiter) -> Poll<()> {
		let Err(err) = ready!(self.carry(waiter)) else {
			return Poll::Ready(());
		};
		self.abort();
		self.failed.lock().expect("failed").get_or_insert(err);
		Poll::Ready(())
	}
}

impl Forward {
	/// Carry new groups, datagrams, and frames, ready once the source has ended and its groups drained.
	///
	/// A source track or group that fails is aborted on the output and the rest carries on; only
	/// encryption errors are returned.
	fn carry(&mut self, waiter: &kio::Waiter) -> Poll<crate::Result<()>> {
		let Some(output) = self.output.as_mut() else {
			return Poll::Ready(Ok(()));
		};

		while !self.ended {
			match self.source.poll_next_group(waiter) {
				Poll::Ready(Ok(Some(source))) => {
					// Joined part way through: frame indices would no longer match the timeline.
					if source.index() != 0 {
						continue;
					}
					match output.create_group(source.sequence) {
						Ok(output) => self.groups.push(Group { source, output }),
						// Below a datagram already carried; encrypting it would reuse that sequence.
						Err(moq_e2ee::Error::Reuse) => continue,
						Err(err) => return Poll::Ready(Err(err.into())),
					}
				}
				Poll::Ready(Ok(None)) => self.ended = true,
				Poll::Ready(Err(err)) => {
					tracing::debug!(track = %self.name, %err, "the plaintext track failed");
					self.abort();
					return Poll::Ready(Ok(()));
				}
				Poll::Pending => break,
			}
		}

		while let Poll::Ready(Ok(Some(datagram))) = self.source.poll_recv_datagram(waiter) {
			match output.insert_datagram(datagram.sequence, datagram.timestamp, &datagram.payload) {
				// Datagrams are lossy: one below a sequence already carried is dropped.
				Ok(()) | Err(moq_e2ee::Error::Reuse) => {}
				Err(err) => return Poll::Ready(Err(err.into())),
			}
		}

		let mut index = 0;
		while index < self.groups.len() {
			let Poll::Ready(end) = self.groups[index].poll(waiter)? else {
				index += 1;
				continue;
			};
			let group = self.groups.swap_remove(index);
			match end {
				None => group.output.finish()?,
				Some(err) => {
					tracing::debug!(track = %self.name, sequence = group.source.sequence, %err, "the plaintext group failed");
					group.output.abort()?;
				}
			}
		}

		if !self.ended || !self.groups.is_empty() {
			return Poll::Pending;
		}
		if let Some(output) = self.output.take() {
			output.finish()?;
		}
		Poll::Ready(Ok(()))
	}

	fn abort(&mut self) {
		for group in self.groups.drain(..) {
			let _ = group.output.abort();
		}
		if let Some(output) = self.output.take() {
			let _ = output.abort();
		}
	}
}

impl Drop for Forward {
	fn drop(&mut self) {
		self.abort();
	}
}

impl Group {
	/// Encrypt every frame read so far, ready with the source's failure once the group ends.
	fn poll(&mut self, waiter: &kio::Waiter) -> Poll<crate::Result<Option<moq_net::Error>>> {
		loop {
			match ready!(self.source.poll_read_frame(waiter)) {
				Ok(Some(frame)) => self.output.write_frame(frame.timestamp, &frame.payload)?,
				Ok(None) => return Poll::Ready(Ok(None)),
				Err(err) => return Poll::Ready(Ok(Some(err))),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hang::catalog::{VideoCodec, VideoConfig};

	fn credential() -> moq_e2ee::Credential {
		moq_e2ee::Credential::new(moq_e2ee::credential::Config {
			context: Bytes::from_static(b"room"),
			kid: 1,
			secret: [7; moq_e2ee::SECRET_LEN],
		})
		.unwrap()
	}

	/// Decrypt the newest group's first frame on the physical track `semantic` names.
	async fn read(
		output: &moq_net::broadcast::Consumer,
		generation: &generation::Consumer,
		semantic: &str,
	) -> moq_e2ee::group::Frame {
		let name = generation.name(semantic).unwrap();
		let subscriber = output.track(name.as_str()).unwrap().subscribe(None).await.unwrap();
		let mut track = generation.consume(subscriber).unwrap();
		let mut group = track.recv_group().await.unwrap().expect("a group");
		group.read_frame().await.unwrap().expect("a frame")
	}

	#[tokio::test]
	async fn carries_a_hang_broadcast_under_physical_names() {
		let credential = credential();
		let generation = credential.produce();
		let reader = credential.consume(generation.epoch().clone());

		let mut local = moq_net::broadcast::Info::new().produce();
		let mut catalog = crate::catalog::Producer::new(&mut local, Default::default()).unwrap();
		let track = local
			.create_track("video0.avc3", catalog.track_info(hang::catalog::PRIORITY.video))
			.unwrap();
		let mut video = catalog
			.video(
				track,
				crate::catalog::hang::Container::Legacy(crate::container::Kind::Video),
				VideoConfig::new(VideoCodec::VP8),
			)
			.unwrap();
		let mut plaintext = video.consume();
		video
			.write(crate::container::Frame {
				timestamp: moq_net::Timestamp::from_micros(1_000).unwrap(),
				duration: None,
				payload: Bytes::from_static(b"keyframe"),
				keyframe: true,
			})
			.unwrap();
		// Never in the catalog, so never carried.
		let stats = local.create_track("stats", None).unwrap();
		stats
			.append_group()
			.unwrap()
			.write_frame(None, Bytes::from_static(b"counters"))
			.unwrap();

		let output = moq_net::broadcast::Info::new().produce();
		let consumer = output.consume();
		let producer = Producer::new(local.consume(), generation.clone(), output.clone()).unwrap();
		let run = tokio::spawn(producer.run());

		let frame = read(&consumer, &reader, hang::Catalog::DEFAULT_NAME).await;
		let protected: hang::Catalog = hang::Catalog::from_slice(&frame.plaintext).unwrap();
		let physical = reader.name("video0.avc3").unwrap().to_string();
		assert_eq!(protected.video.renditions.keys().collect::<Vec<_>>(), [&physical]);
		let timelines = &protected.archive.as_ref().expect("timelines").timelines;
		assert_eq!(
			timelines[&physical],
			reader.name("video0.avc3.timeline.z").unwrap().to_string()
		);

		let compressed = read(&consumer, &reader, hang::Catalog::COMPRESSED_NAME).await;
		let inflated = moq_flate::Decoder::new().frame(&compressed.plaintext).unwrap();
		assert_eq!(inflated, frame.plaintext);

		// The media decrypts to the plaintext publisher's bytes, at the same timestamp.
		let Some(mut group) = plaintext.recv_group().await.unwrap() else {
			panic!("a plaintext group");
		};
		let expected = group.read_frame().await.unwrap().expect("a plaintext frame");
		let media = read(&consumer, &reader, "video0.avc3").await;
		assert_eq!(media.plaintext, expected.payload);
		assert_eq!(media.timestamp, expected.timestamp);

		// No semantic name, plaintext catalog, or uncatalogued track reaches the output.
		for name in [
			"catalog.json",
			"catalog.json.z",
			moq_msf::DEFAULT_NAME,
			"video0.avc3",
			"video0.avc3.timeline.z",
			"stats",
			reader.name("stats").unwrap().as_str(),
		] {
			assert!(
				matches!(consumer.track(name), Err(moq_net::Error::NotFound)),
				"{name} reached the output"
			);
		}

		drop(video);
		drop(stats);
		catalog.finish().unwrap();
		run.await.unwrap().unwrap();
	}

	#[tokio::test]
	async fn refuses_a_catalog_delta() {
		let generation = credential().produce();
		let local = moq_net::broadcast::Info::new().produce();
		let track = local
			.create_track(hang::Catalog::DEFAULT_NAME, hang::Catalog::default_track_info())
			.unwrap();
		let mut group = track.append_group().unwrap();
		let at = moq_net::Timestamp::from_micros(0).unwrap();
		group.write_frame(at, Bytes::from_static(b"{}")).unwrap();
		group.write_frame(at, Bytes::from_static(b"{}")).unwrap();

		let output = moq_net::broadcast::Info::new().produce();
		let producer = Producer::new(local.consume(), generation, output).unwrap();
		assert!(matches!(
			producer.run().await,
			Err(crate::Error::Unencryptable("a catalog delta"))
		));
	}
}
