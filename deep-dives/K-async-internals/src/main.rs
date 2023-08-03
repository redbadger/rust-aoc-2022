use std::{
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread::{self, panicking},
    time::Duration,
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
    Future,
};

fn main() {
    println!("Main: Starting");

    let (executor, spawner) = new_executor_and_spawner();

    println!("Main: Spawning sleep_twice");

    spawner.spawn(sleep_twice(Duration::from_secs(3)));
    drop(spawner);

    println!("Main: Running executor");

    executor.run();

    println!("Main: Done");
}

async fn sleep_twice(duration: Duration) {
    println!("Sleep: Awaiting TimerFuture");
    TimerFuture::new(duration).await;
    println!("Sleep: First sleep done.");
    TimerFuture::new(duration).await;
    println!("Sleep: Second sleep done, returning nothing");
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

// FIXME: this does not work yet!
//
// struct SimpleJoin<FutA, FutB> {
//     a: Option<FutA>,
//     b: Option<FutB>,
// }

// impl<FutA, FutB> Future for SimpleJoin<FutA, FutB>
// where
//     FutA: Future<Output = ()>,
//     FutB: Future<Output = ()>,
// {
//     type Output = ();

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let mut all_done = true;

//         if let Some(fut_a) = &mut self.a {
//             let mut pinned_a = Pin::new(fut_a);
//             pinned_a.poll()
//         }

//         if let Some(fut_b) = self.b {
//             let pinned_b = Box::pin(fut_b);
//         }

//         Poll::Pending
//     }
// }

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        println!(
            "TimerFuture: Creating TimerFuture which will wait for {} seconds.",
            duration.as_secs_f32()
        );

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!("TimerFuture (thread): Timer thread started, going to sleep");
            thread::sleep(duration);
            println!("TimerFuture (thread): Sleep completed");
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!("TimerFuture (thread): Waking up the task");
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("TimerFuture: Timer future polled");
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("TimerFuture: TimerFuture complete, poll returns 'ready'");
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            println!(
                "TimerFuture: TimerFuture not complete, remembering waker and returning 'pending'"
            );
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        println!("Spawner: Spawning a future");
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!("Spawner: Submitting new future to executor");
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        println!("Task: Submitting myself for polling.");
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        println!("Executor: Starting");

        while let Ok(task) = self.ready_queue.recv() {
            println!("Executor: Task ready");
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                println!("Executor: Polling the future");
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    println!("Executor: Future not yet ready");
                    *future_slot = Some(future);
                }
            }
        }

        println!("Executor: No more tasks to run, finishing");
    }
}
