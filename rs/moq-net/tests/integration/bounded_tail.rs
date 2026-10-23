#![cfg(feature = "async")]

//! A request bound ends delivery, without declaring the source group complete.

use crate::support::harness::{MockConnectOptions, connect_mock, spawn};
use moq_net::{Hop, Timescale, Timestamp, Version, origin, track};
use std::time::Duration;

fn origin(hop: u64) -> origin::Producer {
	let (origin, driver) = origin::Producer::new(origin::Config::new(Hop::new(hop).unwrap()));
	spawn(driver);
	origin
}

fn range(start: u64, end: u64) -> track::Subscription {
	track::Subscription::default().with_ranges([track::Range {
		start: track::Anchor::Absolute(track::Position { group: 5, frame: start }),
		end: Some(track::Anchor::Absolute(track::Position { group: 5, frame: end })),
	}])
}

async fn bounded_tail(version: Option<Version>, compressed: bool, frames: u64) {
	let source = origin(1);
	let relay = origin(2);
	let reader = origin(3);
	let broadcast = source.create_broadcast("bcast").unwrap();
	let producer = broadcast
		.create_track(
			"data",
			track::Info::default()
				.with_timescale(Timescale::MILLI)
				.with_compress(compressed),
		)
		.unwrap();
	broadcast.announce(Default::default()).unwrap();
	let mut links = Vec::new();
	let remote = if let Some(version) = version {
		for (upstream, downstream) in [(&source, &relay), (&relay, &reader)] {
			let mut options = MockConnectOptions::new(version);
			options.server_publish = Some(upstream.consume());
			options.client_subscribe = Some(downstream.clone());
			links.push(connect_mock(options).await);
		}
		reader.consume().routed("bcast").await.unwrap();
		reader.consume().request_broadcast("bcast", None).await.unwrap()
	} else {
		broadcast.consume()
	};
	let track = remote.track("data").unwrap();
	let mut first = track.subscribe(range(0, 3)).await.unwrap();
	crate::support::harness::subscribed(&producer).await;
	let mut group = producer.create_group(moq_net::group::Info { sequence: 5 }).unwrap();
	for index in 0..frames {
		group
			.write_frame(Timestamp::from_millis(index).unwrap(), vec![index as u8; 100])
			.unwrap();
	}
	group.finish().unwrap();
	let mut prefix = first.recv_group().await.unwrap().unwrap();
	for index in 0..3.min(frames) {
		assert_eq!(
			prefix.read_frame().await.unwrap().unwrap().payload,
			vec![index as u8; 100]
		);
	}
	assert!(prefix.read_frame().await.unwrap().is_none());
	let mut second = track.subscribe(range(3, 5)).await.unwrap();
	let mut tail = second.recv_group().await.unwrap().unwrap();
	// IETF passthrough readers expose frame ordinals independently of Object IDs;
	// narrow this cursor too so that a stale prefix cannot hide the missing tail.
	tail.set_frames(3..5);
	for index in 3..5.min(frames) {
		let frame = tail.read_frame().await.unwrap().expect("the requested tail frame");
		assert_eq!(frame.payload, vec![index as u8; 100]);
	}
	assert!(tail.read_frame().await.unwrap().is_none());
	drop((tail, prefix, second, first, links));
}

#[moq_clock::test]
async fn a_bounded_plain_copy_accepts_its_tail() {
	moq_clock::timeout(
		Duration::from_secs(1),
		bounded_tail(Some("moq-lite-07-wip".parse().unwrap()), false, 5),
	)
	.await
	.expect("a later subscriber receives the plain tail");
}

#[moq_clock::test]
async fn a_bounded_compressed_copy_accepts_its_tail() {
	moq_clock::timeout(
		Duration::from_secs(1),
		bounded_tail(Some("moq-lite-07-wip".parse().unwrap()), true, 5),
	)
	.await
	.expect("a later subscriber receives and inflates the compressed tail");
}

#[moq_clock::test]
async fn a_bounded_draft20_copy_accepts_its_tail() {
	moq_clock::timeout(
		Duration::from_secs(1),
		bounded_tail(Some("moq-transport-20".parse().unwrap()), false, 5),
	)
	.await
	.expect("a later subscriber receives the transport tail");
}

#[moq_clock::test]
async fn a_bounded_draft21_copy_accepts_its_tail() {
	moq_clock::timeout(
		Duration::from_secs(1),
		bounded_tail(Some("moq-transport-21".parse().unwrap()), false, 5),
	)
	.await
	.expect("a later subscriber receives the transport tail");
}

#[moq_clock::test]
async fn a_bounded_draft22_copy_accepts_its_tail() {
	moq_clock::timeout(
		Duration::from_secs(1),
		bounded_tail(Some("moq-transport-22".parse().unwrap()), false, 5),
	)
	.await
	.expect("a later subscriber receives the transport tail");
}

#[moq_clock::test]
async fn local_bounded_readers_keep_the_same_content() {
	for compressed in [false, true] {
		moq_clock::timeout(Duration::from_secs(1), bounded_tail(None, compressed, 5))
			.await
			.expect("local readers receive the tail");
	}
}

#[moq_clock::test]
async fn a_group_ending_before_or_at_the_request_bound_ends_cleanly() {
	for compressed in [false, true] {
		for frames in [2, 3] {
			moq_clock::timeout(
				Duration::from_secs(1),
				bounded_tail(Some("moq-lite-07-wip".parse().unwrap()), compressed, frames),
			)
			.await
			.expect("a real end remains readable after demand widens");
		}
	}
}
