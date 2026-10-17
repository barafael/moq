//go:build unix

// Spike for quest/m2/ffi-poll.md: today's Task reads against Shape A and Shape B,
// on the generated bindings directly. Prototype only; not for merge.
package moq_test

import (
	"context"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
	"testing"
	"time"

	ffi "moq.dev/moq-ffi/moq"
)

const spikeFrames = 64

// chanWaker is Shape B's Go waker: a wake is a non-blocking send.
type chanWaker chan struct{}

func (w chanWaker) Wake() {
	select {
	case w <- struct{}{}:
	default:
	}
}

// pollGroup is Shape B's wrapper around poll_recv_group: poll, then park on the waker or ctx.
func pollGroup(ctx context.Context, track *ffi.MoqPollTrack, waker chanWaker) (*ffi.MoqPollGroup, error) {
	for {
		result, err := track.PollRecvGroup(waker)
		if err != nil {
			return nil, err
		}
		if ready, ok := result.(ffi.MoqPollGroupResultReady); ok {
			if ready.Group == nil {
				return nil, nil
			}
			return *ready.Group, nil
		}
		select {
		case <-waker:
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	}
}

func pollFrame(ctx context.Context, group *ffi.MoqPollGroup, waker chanWaker) (*ffi.MoqFrame, error) {
	for {
		result, err := group.PollReadFrame(waker)
		if err != nil {
			return nil, err
		}
		if ready, ok := result.(ffi.MoqPollFrameResultReady); ok {
			return ready.Frame, nil
		}
		select {
		case <-waker:
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	}
}

// pollGroupReg and pollFrameReg are Shape B': the waker was registered once with SetWaker.
func pollGroupReg(ctx context.Context, track *ffi.MoqPollTrack, waker chanWaker) (*ffi.MoqPollGroup, error) {
	for {
		result, err := track.PollRecvGroupRegistered()
		if err != nil {
			return nil, err
		}
		if ready, ok := result.(ffi.MoqPollGroupResultReady); ok {
			if ready.Group == nil {
				return nil, nil
			}
			return *ready.Group, nil
		}
		select {
		case <-waker:
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	}
}

func pollFrameReg(ctx context.Context, group *ffi.MoqPollGroup, waker chanWaker) (*ffi.MoqFrame, error) {
	for {
		result, err := group.PollReadFrameRegistered()
		if err != nil {
			return nil, err
		}
		if ready, ok := result.(ffi.MoqPollFrameResultReady); ok {
			return ready.Frame, nil
		}
		select {
		case <-waker:
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	}
}

// bridged is today's wrapper pattern (internal/bridge.run): the call runs on its own goroutine
// and the caller races it against ctx.
func bridged[T any](ctx context.Context, call func() (T, error)) (T, error) {
	type result struct {
		val T
		err error
	}
	ch := make(chan result, 1)
	go func() {
		val, err := call()
		ch <- result{val, err}
	}()
	select {
	case r := <-ch:
		return r.val, r.err
	case <-ctx.Done():
		var zero T
		return zero, ctx.Err()
	}
}

// reader reads one group of spikeFrames frames, checking each.
type reader func(ctx context.Context, n int, payload []byte, ack func()) error

func check(frame *ffi.MoqFrame, n, f int, payload []byte) error {
	if frame == nil {
		return errors.New("group ended")
	}
	if string(frame.Payload) != string(payload) || frame.TimestampUs == nil || *frame.TimestampUs != uint64(n*spikeFrames+f) {
		return fmt.Errorf("corrupt frame %d of round %d", f, n)
	}
	return nil
}

func spikeReader(shape string, consumer *ffi.MoqTrackConsumer) (reader, error) {
	switch shape {
	case "task", "task+bridge":
		wrap := func(ctx context.Context, call func() (*ffi.MoqFrame, error)) (*ffi.MoqFrame, error) {
			if shape == "task" {
				return call()
			}
			return bridged(ctx, call)
		}
		return func(ctx context.Context, n int, payload []byte, ack func()) error {
			group, err := consumer.RecvGroup(ctx)
			if err != nil || group == nil {
				return fmt.Errorf("recv: %v", err)
			}
			defer (*group).Destroy()
			for f := range spikeFrames {
				frame, err := wrap(ctx, func() (*ffi.MoqFrame, error) { return (*group).ReadFrame(ctx) })
				if err != nil {
					return err
				}
				if err := check(frame, n, f, payload); err != nil {
					return err
				}
				ack()
			}
			return nil
		}, nil
	case "a":
		track, err := consumer.IntoPoll()
		if err != nil {
			return nil, err
		}
		return func(ctx context.Context, n int, payload []byte, ack func()) error {
			group, err := track.RecvGroup(ctx)
			if err != nil || group == nil {
				return fmt.Errorf("recv: %v", err)
			}
			defer (*group).Destroy()
			for f := range spikeFrames {
				frame, err := (*group).ReadFrame(ctx)
				if err != nil {
					return err
				}
				if err := check(frame, n, f, payload); err != nil {
					return err
				}
				ack()
			}
			return nil
		}, nil
	case "b":
		track, err := consumer.IntoPoll()
		if err != nil {
			return nil, err
		}
		waker := make(chanWaker, 1)
		return func(ctx context.Context, n int, payload []byte, ack func()) error {
			group, err := pollGroup(ctx, track, waker)
			if err != nil || group == nil {
				return fmt.Errorf("recv: %v", err)
			}
			defer group.Destroy()
			for f := range spikeFrames {
				frame, err := pollFrame(ctx, group, waker)
				if err != nil {
					return err
				}
				if err := check(frame, n, f, payload); err != nil {
					return err
				}
				ack()
			}
			return nil
		}, nil
	case "b1":
		track, err := consumer.IntoPoll()
		if err != nil {
			return nil, err
		}
		waker := make(chanWaker, 1)
		track.SetWaker(waker)
		return func(ctx context.Context, n int, payload []byte, ack func()) error {
			group, err := pollGroupReg(ctx, track, waker)
			if err != nil || group == nil {
				return fmt.Errorf("recv: %v", err)
			}
			defer group.Destroy()
			group.SetWaker(waker)
			for f := range spikeFrames {
				frame, err := pollFrameReg(ctx, group, waker)
				if err != nil {
					return err
				}
				if err := check(frame, n, f, payload); err != nil {
					return err
				}
				ack()
			}
			return nil
		}, nil
	}
	return nil, fmt.Errorf("unknown shape %q", shape)
}

func spikeList(name, def string) []string {
	if v := os.Getenv(name); v != "" {
		return strings.Split(v, ",")
	}
	return strings.Split(def, ",")
}

func spikeThroughput(t *testing.T, shape string, subscribers, size int, lockstep bool, budget time.Duration) {
	ctx := context.Background()
	cache := uint64(64 << 20)
	origin := ffi.NewMoqOriginProducer(ffi.MoqOriginConfig{CacheCapacityBytes: &cache})
	broadcast, err := origin.CreateBroadcast("bench")
	if err != nil {
		t.Fatal(err)
	}
	timescale := uint64(1_000_000)
	track, err := broadcast.PublishTrack("0", &ffi.MoqTrackInfo{Priority: 127, Timescale: &timescale})
	if err != nil {
		t.Fatal(err)
	}
	readers := make([]reader, subscribers)
	for i := range readers {
		consumer, err := track.Consume(nil)
		if err != nil {
			t.Fatal(err)
		}
		if readers[i], err = spikeReader(shape, consumer); err != nil {
			t.Fatal(err)
		}
	}
	payload := make([]byte, size)
	for i := range payload {
		payload[i] = byte(i % 251)
	}
	// Lockstep: the writer waits for every reader to take each frame, so each read parks first.
	acks := make(chan struct{}, subscribers)
	ack := func() {}
	if lockstep {
		ack = func() { acks <- struct{}{} }
	}
	round := func(n int) {
		var wg sync.WaitGroup
		errs := make(chan error, subscribers)
		for _, read := range readers {
			wg.Go(func() {
				if err := read(ctx, n, payload, ack); err != nil {
					errs <- err
					for range spikeFrames {
						ack()
					}
				}
			})
		}
		group, err := track.AppendGroup()
		if err != nil {
			t.Fatal(err)
		}
		for f := range spikeFrames {
			ts := uint64(n*spikeFrames + f)
			if err := group.WriteFrame(ffi.MoqFrame{Payload: payload, TimestampUs: &ts}); err != nil {
				t.Fatal(err)
			}
			if lockstep {
				for range subscribers {
					<-acks
				}
			}
		}
		if err := group.Finish(); err != nil {
			t.Fatal(err)
		}
		wg.Wait()
		close(errs)
		for err := range errs {
			t.Fatal(err)
		}
	}
	for n := range 3 {
		round(n)
	}
	ffi.SpikeCounters()
	start, cpuStart, n := time.Now(), cpuTime(), 3
	for time.Since(start) < budget {
		round(n)
		n++
	}
	wall, cpu := time.Since(start), cpuTime()-cpuStart
	counters := ffi.SpikeCounters()
	rounds := n - 3
	frames := float64(rounds * subscribers * spikeFrames)
	calls := float64(rounds * subscribers * (spikeFrames + 1))
	mode := "burst"
	if lockstep {
		mode = "lockstep"
	}
	name := fmt.Sprintf("%s/%s/size=%d/sub=%d", shape, mode, size, subscribers)
	fmt.Printf("SPIKE throughput/%s MB/s %.2f\n", name, frames*float64(size)/1e6/wall.Seconds())
	fmt.Printf("SPIKE cpu/%s us/frame %.3f\n", name, float64(cpu.Microseconds())/frames)
	fmt.Printf("SPIKE polls/%s per-call %.3f\n", name, float64(counters.Polls)/calls)
	fmt.Printf("SPIKE wakes/%s per-call %.3f\n", name, float64(counters.Wakes)/calls)
}

func spikeLatency(t *testing.T, shape string) {
	producer, err := ffi.NewMoqBroadcastProducer()
	if err != nil {
		t.Fatal(err)
	}
	track, err := producer.PublishTrack("calls", nil)
	if err != nil {
		t.Fatal(err)
	}
	const count, groups = 1000, 10
	ctx := context.Background()
	var elapsed time.Duration
	for range groups {
		group, err := track.AppendGroup()
		if err != nil {
			t.Fatal(err)
		}
		for range count {
			if err := group.WriteFrame(ffi.MoqFrame{Payload: []byte("x")}); err != nil {
				t.Fatal(err)
			}
		}
		consumer, err := group.Consume()
		if err != nil {
			t.Fatal(err)
		}
		var read func() error
		switch shape {
		case "task":
			read = func() error { _, err := consumer.ReadFrame(ctx); return err }
		case "task+bridge":
			read = func() error {
				_, err := bridged(ctx, func() (*ffi.MoqFrame, error) { return consumer.ReadFrame(ctx) })
				return err
			}
		case "a", "b", "b1":
			poll, err := consumer.IntoPoll()
			if err != nil {
				t.Fatal(err)
			}
			waker := make(chanWaker, 1)
			read = func() error { _, err := poll.ReadFrame(ctx); return err }
			if shape == "b" {
				read = func() error { _, err := pollFrame(ctx, poll, waker); return err }
			}
			if shape == "b1" {
				poll.SetWaker(waker)
				read = func() error { _, err := pollFrameReg(ctx, poll, waker); return err }
			}
		}
		start := time.Now()
		for range count {
			if err := read(); err != nil {
				t.Fatal(err)
			}
		}
		elapsed += time.Since(start)
	}
	fmt.Printf("SPIKE call/%s/read_frame_ready ns %.0f\n", shape, float64(elapsed.Nanoseconds())/(count*groups))
}

// TestFFIPollSpike runs only with SPIKE=1 (sweeps in-process, shapes interleaved per point).
func TestFFIPollSpike(t *testing.T) {
	if os.Getenv("SPIKE") == "" {
		t.Skip("set SPIKE=1")
	}
	shapes := spikeList("SPIKE_SHAPES", "task,task+bridge,a,b,b1")
	seconds, _ := strconv.ParseFloat(os.Getenv("MOQ_BENCH_SECONDS"), 64)
	if seconds == 0 {
		seconds = 1
	}
	budget := time.Duration(seconds * float64(time.Second))
	size, _ := strconv.Atoi(os.Getenv("SPIKE_SIZE"))
	if size == 0 {
		size = 1024
	}
	repeats, _ := strconv.Atoi(os.Getenv("SPIKE_REPEATS"))
	for range max(repeats, 1) {
		for _, shape := range shapes {
			spikeLatency(t, shape)
		}
		for _, lockstep := range []bool{false, true} {
			for _, sub := range spikeList("SPIKE_SUBSCRIBERS", "1,4,16,64") {
				subscribers, _ := strconv.Atoi(sub)
				for _, shape := range shapes {
					spikeThroughput(t, shape, subscribers, size, lockstep, budget)
				}
			}
		}
	}
}

// TestFFIPollSpikeCancel cancels a parked read at each delay after its frame lands, then reads
// again: a cancelled read that already took the frame swallows it. Go's generated async call
// cancels the RustFuture and drops a ready result.
func TestFFIPollSpikeCancel(t *testing.T) {
	if os.Getenv("SPIKE") == "" {
		t.Skip("set SPIKE=1")
	}
	for _, shape := range []string{"task", "a", "b"} {
		swallowed, cancelled := 0, 0
		// cancelled counts reads that returned the cancel; swallowed, those whose frame the next read missed.
		for i := range 400 {
			producer, _ := ffi.NewMoqBroadcastProducer()
			track, _ := producer.PublishTrack("t", nil)
			group, _ := track.AppendGroup()
			consumer, _ := group.Consume()
			var read func(ctx context.Context) (*ffi.MoqFrame, error)
			switch shape {
			case "task":
				read = consumer.ReadFrame
			case "a":
				poll, _ := consumer.IntoPoll()
				read = poll.ReadFrame
			case "b":
				poll, _ := consumer.IntoPoll()
				waker := make(chanWaker, 1)
				read = func(ctx context.Context) (*ffi.MoqFrame, error) { return pollFrame(ctx, poll, waker) }
			}
			ctx, cancel := context.WithCancel(context.Background())
			done := make(chan error, 1)
			go func() {
				_, err := read(ctx)
				done <- err
			}()
			time.Sleep(time.Millisecond)
			if i%2 == 0 {
				// A parked read cancelled before its frame lands.
				cancel()
				_ = group.WriteFrame(ffi.MoqFrame{Payload: []byte{1}})
			} else {
				_ = group.WriteFrame(ffi.MoqFrame{Payload: []byte{1}})
				// Spread the cancel across the window where the read is being woken and completed.
				time.Sleep(time.Duration(i%40) * time.Microsecond)
				cancel()
			}
			if err := <-done; err == nil {
				continue
			}
			cancelled++
			_ = group.WriteFrame(ffi.MoqFrame{Payload: []byte{2}})
			ctx2, cancel2 := context.WithTimeout(context.Background(), 2*time.Second)
			frame, err := read(ctx2)
			cancel2()
			if err != nil {
				t.Fatalf("%s: second read: %v", shape, err)
			}
			if frame.Payload[0] != 1 {
				swallowed++
			}
		}
		fmt.Printf("SPIKE cancel/%s cancelled=%d swallowed=%d of 400\n", shape, cancelled, swallowed)
	}
}
