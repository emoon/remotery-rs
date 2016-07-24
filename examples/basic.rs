extern crate remotery;

use remotery::{Remotery, RemoteryScope, SampleFlags};
use std::time::Duration;
use std::thread;

fn some_function() {
    let _scope = RemoteryScope::new("some_function", SampleFlags::Default);
    thread::sleep(Duration::from_millis(10));
}

fn main() {
    let _remotery = Remotery::create_global_instance().unwrap_or_else(|e| {
    	panic!(e);
	});

    for _ in 0..1000 {
        Remotery::log_text("Doing profiling!");
        Remotery::begin_cpu_sample("test", SampleFlags::Default);
        thread::sleep(Duration::from_millis(20));
        some_function();
        Remotery::end_cpu_sample();
    }
}

