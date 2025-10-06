//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

// This approach ensures that the Rc value is dropped before the await point, so it doesn’t need to be sent across threads.
// - Rc is not Send, so it cannot safely cross thread boundaries.
// - When an async fn hits an await, the compiler assumes the task might pause and resume on another thread.
// - By placing Rc inside a block {}, it gets dropped before the await, preventing non-Send data from being held across an await and allowing the code to compile safely.
async fn example() {
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}
