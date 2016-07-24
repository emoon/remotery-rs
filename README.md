# remotery-rs [![Build Status](https://travis-ci.org/emoon/remotery-rs.svg?branch=master)](https://travis-ci.org/emoon/remotery-rs) [![Build status](https://ci.appveyor.com/api/projects/status/ipdufbh57655ln1q?svg=true)](https://ci.appveyor.com/project/emoon/remotery-rs) [![Crates.io](https://img.shields.io/crates/v/remotery.svg)](https://crates.io/crates/remotery)

Remotery is a realtime CPU/GPU profiler with a viewer that runs in a web browser. This lib is a [Rust](https://www.rust-lang.org) wrapper around the C API provided by Remotery and the original repo over here https://github.com/Celtoys/Remotery where more information can be found of how to use the UI. Notice that the remotery-rs only support CPU sampling currently.

![screenshot](https://github.com/Celtoys/Remotery/raw/master/screenshot.png?raw=true)

[Documentation](http://prodbg.com/remotery/remotery/index.html)

Usage
-----

```toml
# Cargo.toml
[dependencies]
remotery = "0.1"
```

Example
-------

```rust
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
```
## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
