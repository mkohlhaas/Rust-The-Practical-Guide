### Cooperative Scheduling

- Cooperative Scheduling, as it gives control back to the executor or runtime, thereby cooperatively allowing other code to make progress and execute

- Unlike threads, async code uses `cooperative scheduling` instead of `preemptive scheduling`.
  - If we have two threads, the operating system can switch between the two threads at any given time.
  - However, in async code, we as developers tell the runtime when a block of async code is ready to yield, so that other async code can run on the same thread.
  - This grants developers more control, but it also increases their responsibility.
  - In particular, we must ensure that our async/await code is efficient.
  - For example, it’s important to avoid placing CPU-intensive operations within an async function.

- The Future trait is fairly complex, but at an abstract level, it has a poll method that can either return a ready state when a return value is available or a pending state indicating that the value is currently not available.
- Futures are lazy. You have to use .await to derive them to completion.
