use futures::executor::{block_on, LocalPool, ThreadPool};

async fn async_funcs() {
    async_func_a().await;
    async_func_b().await;
    async_func_c().await;
}

async fn async_func_a() {
    println!("a");
}

async fn async_func_b() {
    println!("b");
}

async fn async_func_c() {
    println!("c");
}

fn wait() {
    let duration = std::time::Duration::from_millis(1000);
    std::thread::sleep(duration);
}

fn main() {
    // 1:1 model
    let future = async_funcs();
    block_on(future);

    wait();

    // 1:N model
    let mut pool = LocalPool::new();
    let future = async_funcs();
    pool.run_until(future);

    wait();

    // M:N model
    let pool = ThreadPool::new().unwrap();
    let future = async_funcs();
    pool.spawn_ok(future);
}
