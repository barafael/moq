//! Spike for quest/m2/ffi-poll.md: two poll-native async shapes on the hot read path,
//! measured against `ffi::Task`. Prototype only; not for merge.
//!
//! - Shape A: `recv_group` / `read_frame` are `kio::wait` over the `poll_*`, polled directly by
//!   uniffi's `RustFuture`. No tokio mutex, no cancel watch, no runtime entry.
//! - Shape B: `poll_recv_group` / `poll_read_frame` are sync calls taking a foreign [`MoqWaker`];
//!   each wrapper builds its own future around them.
//! - Shape B': as B, but the waker is registered once per handle (`set_waker`), so a poll
//!   lowers no callback handle.
//!
//! Reproduce (see the PR for results):
//! - `cargo test -p moq-ffi --lib spike::tests -- --nocapture`, then again with
//!   `--config 'patch.crates-io.uniffi_core.path="<uniffi_core 0.32.2 with spike-uniffi-core-cancel.patch>"'`
//! - Go: stage with `MOQ_FFI_PROFILE=release sh/go/stage.sh --output dist/spike`, then in the staged
//!   wrapper `SPIKE=1 SPIKE_REPEATS=3 go test -run TestFFIPollSpike -v .`
//! - Python: `maturin develop --release` in py/moq-ffi, then `python moq-rs/benches/ffi_poll.py`
//!   (`SPIKE_OUT=py.json`) and `python moq-rs/benches/ffi_poll_cancel.py`
//! - `bench/ffi_poll_agg.py` turns the Go log and Python JSON (under `.spike/`) into the tables.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

use crate::consumer::{MoqGroupConsumer, MoqTrackConsumer};
use crate::error::MoqError;
use crate::media::MoqFrame;

/// Rust-side polls of the hot-path read futures (every shape), and foreign wakes (Shape B).
pub(crate) static POLLS: AtomicU64 = AtomicU64::new(0);
static WAKES: AtomicU64 = AtomicU64::new(0);

/// Counters since the last call.
#[derive(uniffi::Record)]
pub struct MoqSpikeCounters {
	pub polls: u64,
	pub wakes: u64,
}

/// Read and reset the spike counters.
#[uniffi::export]
pub fn spike_counters() -> MoqSpikeCounters {
	MoqSpikeCounters {
		polls: POLLS.swap(0, Ordering::Relaxed),
		wakes: WAKES.swap(0, Ordering::Relaxed),
	}
}

/// A foreign wake-up, called from any thread once a pending `poll_*` can progress.
#[uniffi::export(with_foreign)]
pub trait MoqWaker: Send + Sync {
	fn wake(&self);
}

struct ForeignWaker(Arc<dyn MoqWaker>);

impl std::task::Wake for ForeignWaker {
	fn wake(self: Arc<Self>) {
		self.wake_by_ref();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		WAKES.fetch_add(1, Ordering::Relaxed);
		self.0.wake();
	}
}

/// What a Shape B poll keeps between calls: the waiter whose registrations the next wake follows.
#[derive(Default)]
struct Parked(Mutex<Option<kio::Waiter>>);

impl Parked {
	/// Poll once with a fresh waiter built from `waker`, keeping it only while pending.
	fn poll<R>(&self, waker: Arc<dyn MoqWaker>, poll: impl FnOnce(&kio::Waiter) -> Poll<R>) -> Option<R> {
		let waiter = kio::Waiter::new(Waker::from(Arc::new(ForeignWaker(waker))));
		let result = poll(&waiter);
		let mut parked = self.0.lock().unwrap();
		match result {
			Poll::Ready(value) => {
				*parked = None;
				Some(value)
			}
			Poll::Pending => {
				// Replacing the old waiter kills its registrations.
				*parked = Some(waiter);
				None
			}
		}
	}
}

/// Shape B': the waker is registered once per handle, so a poll lowers no callback handle.
#[derive(Default)]
struct Registered {
	waker: Mutex<Option<Waker>>,
	park: Mutex<kio::Park>,
}

impl Registered {
	fn set(&self, waker: Arc<dyn MoqWaker>) {
		*self.waker.lock().unwrap() = Some(Waker::from(Arc::new(ForeignWaker(waker))));
	}

	fn poll<R>(&self, poll: impl FnOnce(&kio::Waiter) -> Poll<R>) -> Result<Option<R>, MoqError> {
		let waker = self.waker.lock().unwrap().clone().ok_or(MoqError::Closed)?;
		let mut park = self.park.lock().unwrap();
		// Reuses the held waiter while the waker is the same, so a steady poll allocates nothing.
		let waiter = park.hold(&std::task::Context::from_waker(&waker));
		Ok(match poll(waiter) {
			Poll::Ready(value) => Some(value),
			Poll::Pending => None,
		})
	}
}

/// Ends every read on [`Self::close`], waking any parked on it.
struct Cancel {
	producer: kio::Producer<()>,
	consumer: kio::Consumer<()>,
}

impl Cancel {
	fn new() -> Self {
		let producer = kio::Producer::new(());
		let consumer = producer.consume();
		Self { producer, consumer }
	}

	fn poll(&self, waiter: &kio::Waiter) -> Poll<()> {
		self.consumer.poll_closed(waiter)
	}

	fn close(&self) {
		let _ = self.producer.close();
	}
}

/// A track subscription read through either poll-native shape.
#[derive(uniffi::Object)]
pub struct MoqPollTrack {
	track: Mutex<Option<moq_net::track::Subscriber>>,
	cancel: Cancel,
	parked: Parked,
	registered: Registered,
}

/// One Shape B poll of [`MoqPollTrack::poll_recv_group`].
#[derive(uniffi::Enum)]
pub enum MoqPollGroupResult {
	Pending,
	Ready { group: Option<Arc<MoqPollGroup>> },
}

/// One Shape B poll of [`MoqPollGroup::poll_read_frame`].
#[derive(uniffi::Enum)]
pub enum MoqPollFrameResult {
	Pending,
	Ready { frame: Option<MoqFrame> },
}

impl MoqPollTrack {
	fn poll_recv(&self, waiter: &kio::Waiter) -> Poll<Result<Option<Arc<MoqPollGroup>>, MoqError>> {
		POLLS.fetch_add(1, Ordering::Relaxed);
		if self.cancel.poll(waiter).is_ready() {
			return Poll::Ready(Err(MoqError::Cancelled));
		}
		let mut track = self.track.lock().unwrap();
		let Some(track) = track.as_mut() else {
			return Poll::Ready(Err(MoqError::Cancelled));
		};
		track.poll_recv_group(waiter).map(|result| {
			result
				.map(|group| group.map(|group| Arc::new(MoqPollGroup::new(group))))
				.map_err(Into::into)
		})
	}
}

#[uniffi::export]
impl MoqTrackConsumer {
	/// Spike: move this subscription into a [`MoqPollTrack`].
	pub fn into_poll(&self) -> Result<Arc<MoqPollTrack>, MoqError> {
		let (track, _driver) = self.take()?;
		Ok(Arc::new(MoqPollTrack {
			track: Mutex::new(Some(track)),
			cancel: Cancel::new(),
			parked: Parked::default(),
			registered: Registered::default(),
		}))
	}
}

#[uniffi::export]
impl MoqGroupConsumer {
	/// Spike: move this group into a [`MoqPollGroup`].
	pub fn into_poll(&self) -> Result<Arc<MoqPollGroup>, MoqError> {
		let (group, _driver) = self.take()?;
		Ok(Arc::new(MoqPollGroup::new(group)))
	}
}

#[uniffi::export]
impl MoqPollTrack {
	/// Shape A: the next group in arrival order, or `None` when the track ends.
	pub async fn recv_group(&self) -> Result<Option<Arc<MoqPollGroup>>, MoqError> {
		kio::wait(|waiter| self.poll_recv(waiter)).await
	}

	/// Shape B: one poll for the next group; `waker` fires once a later poll can progress.
	pub fn poll_recv_group(&self, waker: Arc<dyn MoqWaker>) -> Result<MoqPollGroupResult, MoqError> {
		match self.parked.poll(waker, |waiter| self.poll_recv(waiter)) {
			None => Ok(MoqPollGroupResult::Pending),
			Some(group) => Ok(MoqPollGroupResult::Ready { group: group? }),
		}
	}

	/// Shape B': the waker every later [`Self::poll_recv_group_registered`] parks on.
	pub fn set_waker(&self, waker: Arc<dyn MoqWaker>) {
		self.registered.set(waker);
	}

	/// Shape B': one poll for the next group, parking on the registered waker.
	pub fn poll_recv_group_registered(&self) -> Result<MoqPollGroupResult, MoqError> {
		match self.registered.poll(|waiter| self.poll_recv(waiter))? {
			None => Ok(MoqPollGroupResult::Pending),
			Some(group) => Ok(MoqPollGroupResult::Ready { group: group? }),
		}
	}

	/// End every current and future read, releasing the subscription.
	pub fn cancel(&self) {
		self.track.lock().unwrap().take();
		self.cancel.close();
	}
}

/// A group read through either poll-native shape.
#[derive(uniffi::Object)]
pub struct MoqPollGroup {
	sequence: u64,
	group: Mutex<Option<moq_net::group::Consumer>>,
	cancel: Cancel,
	parked: Parked,
	registered: Registered,
}

impl MoqPollGroup {
	fn new(group: moq_net::group::Consumer) -> Self {
		Self {
			sequence: group.sequence,
			group: Mutex::new(Some(group)),
			cancel: Cancel::new(),
			parked: Parked::default(),
			registered: Registered::default(),
		}
	}

	fn poll_read(&self, waiter: &kio::Waiter) -> Poll<Result<Option<MoqFrame>, MoqError>> {
		POLLS.fetch_add(1, Ordering::Relaxed);
		if self.cancel.poll(waiter).is_ready() {
			return Poll::Ready(Err(MoqError::Cancelled));
		}
		let mut group = self.group.lock().unwrap();
		let Some(group) = group.as_mut() else {
			return Poll::Ready(Err(MoqError::Cancelled));
		};
		let frame = match std::task::ready!(group.poll_read_frame(waiter)) {
			Ok(frame) => frame,
			Err(err) => return Poll::Ready(Err(err.into())),
		};
		Poll::Ready(frame.map(crate::consumer::raw_frame).transpose())
	}
}

#[uniffi::export]
impl MoqPollGroup {
	/// The sequence number of this group within the track.
	pub fn sequence(&self) -> u64 {
		self.sequence
	}

	/// Shape A: the next frame, or `None` when the group ends.
	pub async fn read_frame(&self) -> Result<Option<MoqFrame>, MoqError> {
		kio::wait(|waiter| self.poll_read(waiter)).await
	}

	/// Shape B: one poll for the next frame; `waker` fires once a later poll can progress.
	pub fn poll_read_frame(&self, waker: Arc<dyn MoqWaker>) -> Result<MoqPollFrameResult, MoqError> {
		match self.parked.poll(waker, |waiter| self.poll_read(waiter)) {
			None => Ok(MoqPollFrameResult::Pending),
			Some(frame) => Ok(MoqPollFrameResult::Ready { frame: frame? }),
		}
	}

	/// Shape B': the waker every later [`Self::poll_read_frame_registered`] parks on.
	pub fn set_waker(&self, waker: Arc<dyn MoqWaker>) {
		self.registered.set(waker);
	}

	/// Shape B': one poll for the next frame, parking on the registered waker.
	pub fn poll_read_frame_registered(&self) -> Result<MoqPollFrameResult, MoqError> {
		match self.registered.poll(|waiter| self.poll_read(waiter))? {
			None => Ok(MoqPollFrameResult::Pending),
			Some(frame) => Ok(MoqPollFrameResult::Ready { frame: frame? }),
		}
	}

	/// End every current and future read, releasing the group.
	pub fn cancel(&self) {
		self.group.lock().unwrap().take();
		self.cancel.close();
	}
}

/// Drive reads through uniffi's `RustFuture` the way a foreign binding does, then cancel one
/// without freeing it: the window Kotlin and Swift leave between `rust_future_cancel` and
/// `rust_future_free`, and the one mozilla/uniffi-rs#2771 is about.
#[cfg(test)]
mod tests {
	use std::sync::{Arc, Mutex};

	use uniffi::{Handle, LiftArgsError, RustFuturePoll};

	use crate::media::MoqFrame;
	use crate::producer::{MoqBroadcastProducer, MoqGroupProducer};

	/// Each continuation, by the `data` its poll was given.
	static CONTINUATIONS: Mutex<Vec<(u64, RustFuturePoll)>> = Mutex::new(Vec::new());

	extern "C" fn continuation(data: u64, poll: RustFuturePoll) {
		CONTINUATIONS.lock().unwrap().push((data, poll));
	}

	fn ready(data: u64) -> bool {
		CONTINUATIONS
			.lock()
			.unwrap()
			.iter()
			.any(|(d, poll)| *d == data && *poll == RustFuturePoll::Ready)
	}

	fn woken(data: u64) -> bool {
		CONTINUATIONS.lock().unwrap().iter().any(|(d, _)| *d == data)
	}

	/// A read as a foreign future: resolves to the frame's first byte, or 0 for none.
	fn start<F>(read: F) -> Handle
	where
		F: std::future::Future<Output = Result<Option<MoqFrame>, crate::error::MoqError>> + Send + 'static,
	{
		uniffi::rust_future_new(
			async move {
				let frame = read.await;
				Ok::<u8, LiftArgsError>(match frame {
					Ok(Some(frame)) => frame.payload[0],
					Ok(None) => 0,
					Err(_) => 255,
				})
			},
			crate::UniFfiTag,
		)
	}

	fn poll(handle: Handle, data: u64) {
		unsafe { uniffi::rust_future_poll::<u8>(handle, continuation, data) }
	}

	fn complete(handle: Handle) -> u8 {
		let mut status = uniffi::RustCallStatus::default();
		let value = unsafe { uniffi::rust_future_complete::<u8>(handle.clone(), &mut status) };
		unsafe { uniffi::rust_future_free::<u8>(handle) };
		value
	}

	fn group() -> Arc<MoqGroupProducer> {
		let broadcast = MoqBroadcastProducer::new().unwrap();
		let track = broadcast.publish_track("t".into(), None).unwrap();
		let group = track.append_group().unwrap();
		// Keep the broadcast and track alive for the rest of the test.
		std::mem::forget((broadcast, track));
		group
	}

	/// Read once, cancel the read while parked without freeing it, then read again on the same
	/// handle. Returns whether the second read got the frame written after the cancel.
	fn second_read_after_cancel(reader: impl Fn() -> Handle, base: u64) -> bool {
		let first = reader();
		poll(first.clone(), base);
		assert!(!woken(base), "the first read parks");
		unsafe { uniffi::rust_future_cancel::<u8>(first.clone()) };
		assert!(ready(base), "cancel resolves the continuation");

		let second = reader();
		poll(second.clone(), base + 1);
		write(base);
		let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
		while !ready(base + 1) && std::time::Instant::now() < deadline {
			if woken(base + 1) {
				CONTINUATIONS.lock().unwrap().retain(|(d, _)| *d != base + 1);
				poll(second.clone(), base + 1);
			}
			std::thread::yield_now();
		}
		let got = ready(base + 1) && complete(second.clone()) == 7;
		if !got {
			unsafe { uniffi::rust_future_free::<u8>(second) };
		}
		unsafe { uniffi::rust_future_free::<u8>(first) };
		got
	}

	thread_local! {
		static WRITER: std::cell::RefCell<Option<Arc<MoqGroupProducer>>> = const { std::cell::RefCell::new(None) };
	}

	fn write(_base: u64) {
		WRITER.with(|w| {
			w.borrow()
				.as_ref()
				.unwrap()
				.write_frame(MoqFrame {
					payload: vec![7],
					timestamp_us: None,
				})
				.unwrap()
		});
	}

	#[test]
	fn cancel_without_free() {
		// Today's Task: the cancelled read keeps the read lane until it is freed.
		let producer = group();
		let consumer = Arc::new(producer.consume().unwrap());
		WRITER.with(|w| *w.borrow_mut() = Some(producer));
		let task = second_read_after_cancel(
			|| {
				let consumer = consumer.clone();
				start(async move { consumer.read_frame().await })
			},
			100,
		);

		// Shape A: no lane, so the second read proceeds whether or not the first is dropped.
		let producer = group();
		let consumer = producer.consume().unwrap().into_poll().unwrap();
		WRITER.with(|w| *w.borrow_mut() = Some(producer));
		let poll_native = second_read_after_cancel(
			|| {
				let consumer = consumer.clone();
				start(async move { consumer.read_frame().await })
			},
			200,
		);

		let core = std::env::var("SPIKE_CORE").unwrap_or_else(|_| "upstream".into());
		println!("uniffi core {core}: Task second read ok = {task}, Shape A second read ok = {poll_native}");
	}
}
