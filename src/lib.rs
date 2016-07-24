//! Remotery is a realtime CPU/GPU profiler with a viewer that runs in a web browser.
//! This lib is a [Rust](https://www.rust-lang.org) wrapper around the C API provided by Remotery and the original
//! repo over here https://github.com/Celtoys/Remotery
//!
pub mod error;
mod remotery_ffi;
mod cfixed_string;
use std::ptr;
use std::os::raw::c_void;
use error::RemoteryError;
use cfixed_string::CFixedString;

/// Holds the main instance for Remotery
pub struct Remotery {
    instance: *mut c_void,
}

#[derive(Clone, Copy)]
/// Sample flags to decide how profiling info should be handled
pub enum SampleFlags {
    /// Default behaviour
	Default,
    /// Search parent for same-named samples and merge timing instead of adding a new sample
	Aggregate,
}

impl Remotery {
	/// Creates the global instance (with in the C lib that this code wraps) this code needs to be
	/// called before any of the other code is being called and the instance will be dropped when
	/// it goes out of scope so it's suggested to call this early in the main entry point of your
	/// program (such as ``main``)
	///
	/// # Examples
	///
	/// ```ignore
    /// let _remotery = Remotery::create_global_instance().unwrap_or_else(|e| {
    ///  	panic!(e);
	/// });
	/// ```
	///
    pub fn create_global_instance() -> Result<Remotery, RemoteryError> {
        let mut instance = 0 as *mut c_void;

        let res = unsafe {
            remotery_ffi::_rmt_CreateGlobalInstance(&mut instance)
        };

        if res != 0 {
            return Err(error::get_error(res));
        }

        Ok(Remotery { instance: instance })
    }

    ///
    /// Begin a cpu sample. Notice that this call needs to be paired with ``end_cpu_sample``.
    /// It's also possible to use ```RemoteryScope``` that will call end_cpu_scope when the scop ends.
    ///
	/// # Examples
	/// ```ignore
    /// Remotery::begin_cpu_sample("my_function", SampleFlags::Default);
    /// // some code to profile here
    /// Remotery::end_cpu_sample();
    /// ```
    ///
    pub fn begin_cpu_sample(name: &str, flags: SampleFlags) {
        // TODO: As we send 0 as last parameter which is hash caching this will always recalculate
        // the hash which adds some slight overhead. Would be nice if that could be solved somehow.
        unsafe {
            let temp_str = CFixedString::from_str(name);
            remotery_ffi::_rmt_BeginCPUSample(temp_str.as_ptr(), flags as u32, ptr::null_mut());
        }
    }

    /// Ends a cpu sample. Notice that this needs to be paired with ``begin_cpu_sample`` as seen above.
    pub fn end_cpu_sample() {
        unsafe {
            remotery_ffi::_rmt_EndCPUSample();
        }
    }

    /// Setts the name of the current thread that makes it easier to identify it in the Remotery UI
    ///
    /// # Examples
    ///
    /// ```ignore
    /// Remotery::set_current_thread_name("my_thread_name");
    /// ```
    ///
    pub fn set_current_thread_name(name: &str) {
        unsafe {
            let temp_str = CFixedString::from_str(name);
            remotery_ffi::_rmt_SetCurrentThreadName(temp_str.as_ptr());
        }
    }

    ///
    /// Can be used to log text to the remotery ui
    ///
    pub fn log_text(text: &str) {
        unsafe {
            let temp_str = CFixedString::from_str(text);
            remotery_ffi::_rmt_LogText(temp_str.as_ptr());
        }
    }
}

/// Scopes allows you to profile a bit of code and the end_cpu_sample will be called once it goes out of scope
pub struct RemoteryScope;

impl RemoteryScope {
	///
	/// Begin a new Scope which auto calls ``end_cpu_scope`` once the scope ends
	///
    /// # Examples
    ///
    /// ```ignore
    /// let _scope = RemoteryScope::new("my_function", SampleFlags::Default);
    /// ```
    pub fn new(name: &str, flags: SampleFlags) -> RemoteryScope {
        Remotery::begin_cpu_sample(name, flags);
        RemoteryScope {}
    }
}

impl Drop for RemoteryScope {
    fn drop(&mut self) {
        Remotery::end_cpu_sample()
    }
}

impl Drop for Remotery {
    fn drop(&mut self) {
        if self.instance == ptr::null_mut() {
            return
        }

        unsafe {
            remotery_ffi::_rmt_DestroyGlobalInstance(self.instance);
        }
    }
}

