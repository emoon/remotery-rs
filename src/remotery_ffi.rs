#![allow(dead_code, non_camel_case_types, non_snake_case)]

use std::os::raw::{c_void, c_char, c_uint, c_ushort};

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum RmtError {
    RMT_ERROR_NONE = 0,
    RMT_ERROR_MALLOC_FAIL = 1,
    RMT_ERROR_TLS_ALLOC_FAIL = 2,
    RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL = 3,
    RMT_ERROR_CREATE_THREAD_FAIL = 4,
    RMT_ERROR_SOCKET_INIT_NETWORK_FAIL = 5,
    RMT_ERROR_SOCKET_CREATE_FAIL = 6,
    RMT_ERROR_SOCKET_BIND_FAIL = 7,
    RMT_ERROR_SOCKET_LISTEN_FAIL = 8,
    RMT_ERROR_SOCKET_SET_NON_BLOCKING_FAIL = 9,
    RMT_ERROR_SOCKET_INVALID_POLL = 10,
    RMT_ERROR_SOCKET_SELECT_FAIL = 11,
    RMT_ERROR_SOCKET_POLL_ERRORS = 12,
    RMT_ERROR_SOCKET_ACCEPT_FAIL = 13,
    RMT_ERROR_SOCKET_SEND_TIMEOUT = 14,
    RMT_ERROR_SOCKET_SEND_FAIL = 15,
    RMT_ERROR_SOCKET_RECV_NO_DATA = 16,
    RMT_ERROR_SOCKET_RECV_TIMEOUT = 17,
    RMT_ERROR_SOCKET_RECV_FAILED = 18,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET = 19,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION = 20,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION = 21,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST = 22,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST = 23,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY = 24,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY = 25,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL = 26,
    RMT_ERROR_WEBSOCKET_DISCONNECTED = 27,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER = 28,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_SIZE = 29,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_MASK = 30,
    RMT_ERROR_WEBSOCKET_RECEIVE_TIMEOUT = 31,
    RMT_ERROR_REMOTERY_NOT_CREATED = 32,
    RMT_ERROR_SEND_ON_INCOMPLETE_PROFILE = 33,
    RMT_ERROR_CUDA_DEINITIALIZED = 34,
    RMT_ERROR_CUDA_NOT_INITIALIZED = 35,
    RMT_ERROR_CUDA_INVALID_CONTEXT = 36,
    RMT_ERROR_CUDA_INVALID_VALUE = 37,
    RMT_ERROR_CUDA_INVALID_HANDLE = 38,
    RMT_ERROR_CUDA_OUT_OF_MEMORY = 39,
    RMT_ERROR_ERROR_NOT_READY = 40,
    RMT_ERROR_D3D11_FAILED_TO_CREATE_QUERY = 41,
    RMT_ERROR_OPENGL_ERROR = 42,
    RMT_ERROR_CUDA_UNKNOWN = 43,
}

pub type rmtMallocPtr = Option<unsafe extern "C" fn(mm_context: *mut c_void, size: c_uint) -> *mut c_void>;
pub type rmtReallocPtr = Option<unsafe extern "C" fn(mm_context: *mut c_void, ptr: *mut c_void, size: c_uint) -> *mut c_void>;
pub type rmtFreePtr = Option<unsafe extern "C" fn(mm_context: *mut c_void, ptr: *mut c_void)>;
pub type rmtInputHandlerPtr = Option<unsafe extern "C" fn(text: *const c_char, context: *mut c_void)>;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct RmtSettings {
    pub port: c_ushort,
    pub limit_connections_to_localhost: c_uint,
    pub msSleepBetweenServerUpdates: c_uint,
    pub messageQueueSizeInBytes: c_uint,
    pub maxNbMessagesPerUpdate: c_uint,
    pub malloc: rmtMallocPtr,
    pub realloc: rmtReallocPtr,
    pub free: rmtFreePtr,
    pub mm_context: *mut c_void,
    pub input_handler: rmtInputHandlerPtr,
    pub input_handler_context: *mut c_void,
    pub logFilename: *const c_char,
}

extern "C" {
    pub fn _rmt_Settings() -> *mut RmtSettings;
    pub fn _rmt_CreateGlobalInstance(remotery: *mut *mut c_void) -> c_uint;
    pub fn _rmt_DestroyGlobalInstance(remotery: *mut c_void);
    pub fn _rmt_SetGlobalInstance(remotery: *mut c_void);
    pub fn _rmt_GetGlobalInstance() -> *mut c_void;
    pub fn _rmt_SetCurrentThreadName(thread_name: *const c_char);
    pub fn _rmt_LogText(text: *const c_char);
    pub fn _rmt_BeginCPUSample(name: *const c_char, flags: c_uint, hash_cache: *mut c_uint);
    pub fn _rmt_EndCPUSample();
}
