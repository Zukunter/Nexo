use std::{env, sync::Arc, thread};
use threadpool::ThreadPool;
mod notify;
mod nexus;

fn main() {
    let args_given_user: Vec<String> = env::args().collect();
    let mut cmds = args_given_user.iter().skip(1).map(|s| s.as_str());

    let max_thread = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
    ;;
    let thread_pool = Arc::new(ThreadPool::new(max_thread));

    let _ = nexus::start_new_outputer_or_directly_write_in(
        None,
        &mut cmds,
        &thread_pool,
        &String::new(),     
        0,
        true,
        false
    );
    // wait until the all thread have ended
    thread_pool.join();
}
