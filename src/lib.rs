mod remotery_ffi;
mod error;
mod cfixed_string;
use std::ptr;
use std::os::raw::c_void;
use error::RemoteryError;
use cfixed_string::CFixedString;

pub struct Remotery {
    instance: *mut c_void,
}

impl Remotery {
    pub fn create_global_instance() -> Result<Remotery, RemoteryError> {
        let mut instance = 0 as *mut c_void;

        let res = unsafe {
            remotery_ffi::_rmt_CreateGlobalInstance(&mut instance)
        };

        if res != 0 {
            return Err(error::get_error(res));
        }

        println!("Created global instance {:?}", instance);

        Ok(Remotery {
            instance: instance,
        })
    }

    pub fn begin_cpu_sample(name: &str, flags: u32) {
        // TODO: As we send 0 as last parameter which is hash caching this will always recalculate
        // the hash which adds some slight overhead. Would be nice if that could be solved somehow.
        unsafe {
            let temp_str = CFixedString::from_str(name);
            remotery_ffi::_rmt_BeginCPUSample(temp_str.as_ptr(), flags, ptr::null_mut());
        }
    }

    pub fn end_cpu_sample() {
        unsafe {
            remotery_ffi::_rmt_EndCPUSample();
        }
    }

    pub fn set_current_thread_name(name: &str) {
        unsafe {
            let temp_str = CFixedString::from_str(name);
            remotery_ffi::_rmt_SetCurrentThreadName(temp_str.as_ptr());
        }
    }

    pub fn log_text(text: &str) {
        unsafe {
            let temp_str = CFixedString::from_str(text);
            remotery_ffi::_rmt_LogText(temp_str.as_ptr());
        }
    }
}

pub struct RemoteryScope(u32);

impl RemoteryScope {
    pub fn new(name: &str, flags: u32) -> RemoteryScope {
        Remotery::begin_cpu_sample(name, flags);
        RemoteryScope(0)
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

