extern crate remotery;

use remotery::{Remotery, RemoteryScope};
use std::time::Duration;
use std::thread;

fn some_function() {
    let _scope = RemoteryScope::new("some_function", 0);
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    println!("start");
    let _remotery = match Remotery::create_global_instance() {
        Ok(inst) => inst,
        Err(e) => {
            println!("Unable to Create Remotery: {}", e);
            return;
        }
    };

    println!("loop");
    for _ in 0..100 {
        Remotery::log_text("Doing profiling!");
        Remotery::begin_cpu_sample("test", 0);
        thread::sleep(Duration::from_millis(100));
        Remotery::end_cpu_sample();
        some_function();
    }
    println!("end");
}

