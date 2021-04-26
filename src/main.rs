use futures::executor::block_on;
use std::sync::{Arc, Mutex};

struct State {
    count: u64
}

async fn second_greet(state: &Arc<Mutex<State>>) -> u64{
    if let Ok(mut state) = state.lock() {
        state.count +=2;
    }
    print!("hello agian, world\n");
    2
}

async fn greeter(state: &Arc<Mutex<State>>) -> u64{
    if let Ok(mut state) = state.lock() {
        state.count +=1;
    }
    println!("hello world\n");
    1
}

async fn async_main() {
    let mut state  = State {
        count: 0,
    };

    let data = Arc::new(Mutex::new(state));

    let (result1, result2) = futures::join!(greeter(&data), second_greet(&data));
    println!("result1-{}, result2-{}", result1, result2);
    if let Ok(s) = data.lock() {
        println!("state: {}",s.count);
    };
}

fn main() {
    block_on(async_main());
}
