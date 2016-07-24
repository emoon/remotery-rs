use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum RemoteryError {
	/// Malloc call within remotery failed
    Malloc = 1,
	/// Attempt to allocate thread local storage failed
    TlsAllocFail = 2,
	/// Failed to create a virtual memory mirror buffer
    VirtualMemoryBufferFail = 3,
	/// Failed to create a thread for the server
    CreateThreadFail = 4,
	/// Network initialisation failure (e.g. on Win32, WSAStartup fails)
    SocketInitNetworkFail = 5,
	/// Can't create a socket for connection to the remote viewer
    SocketCreateFail = 6,
	/// Can't bind a socket for the server
    SocketBindFail = 7,
	/// Created server socket failed to enter a listen state
    SocketListenFail = 8,
	/// Created server socket failed to switch to a non-blocking state
    SocketSetNonBlockingFail = 9,
	/// Poll attempt on an invalid socket
    SocketInvalidPoll = 10,
	/// Server failed to call select on socket
    SocketSelectFail = 11,
	/// Poll notified that the socket has errors
    SocketPollErrors = 12,
	/// Server failed to accept connection from client
    SocketAcceptFail = 13,
	/// Timed out trying to send data
    SocketSendTimeout = 14,
	/// Unrecoverable error occured while client/server tried to send data
    SocketSendFail = 15,
	/// No data available when attempting a receive
    SocketRecvNoData = 16,
	/// Timed out trying to receive data
    SocketRecvTimeout = 17,
	/// Unrecoverable error occured while client/server tried to receive data
    SocketRecvFailed = 18,
	/// WebSocket server handshake failed, not HTTP GET
    WebsocketHandshakeNotGet = 19,
	/// WebSocket server handshake failed, can't locate WebSocket version
    WebsocketHandshakeNoVersion = 20,
	/// WebSocket server handshake failed, unsupported WebSocket version
    WebsocketHandshakeBadVersion = 21,
	/// WebSocket server handshake failed, can't locate host
    WebsocketHandshakeNoHost = 22,
	/// WebSocket server handshake failed, host is not allowed to connect
    WebsocketHandshakeBadHost = 23,
	/// WebSocket server handshake failed, can't locate WebSocket key
    WebsocketHandshakeNoKey = 24,
	/// WebSocket server handshake failed, WebSocket key is ill-formed
    WebsocketHandshakeBadKey = 25,
	/// WebSocket server handshake failed, internal error, bad string code
    WebsocketHandshakeStringFail = 26,
	/// WebSocket server received a disconnect request and closed the socket
    WebsocketDisconnected = 27,
	/// Couldn't parse WebSocket frame header
    WebsocketBadFrameHeader = 28,
	/// Partially received wide frame header size
    WebsocketBadFrameHeaderSize = 29,
	/// Partially received frame header data mask
    WebsocketBadFrameHeaderMask = 30,
	/// Timeout receiving frame header
    WebsocketReceiveTimeout = 31,
	/// Remotery object has not been created
    RemoteryNotCreated = 32,
	/// An attempt was made to send an incomplete profile tree to the client
    SendOnIncompleteProfile = 33,
	/// This indicates that the CUDA driver is in the process of shutting down
    CudaDeinitialized = 34,
	/// This indicates that the CUDA driver has not been initialized with cuInit() or that initialization has failed
    CudaNotInitialized = 35,
	/// This most frequently indicates that there is no context bound to the current thread
    CudaInvalidContext = 36,
	/// This indicates that one or more of the parameters passed to the API call is not within an acceptable range of values
    CudaInvalidValue = 37,
	/// This indicates that a resource handle passed to the API call was not valid
    CudaInvalidHandle = 38,
	/// The API call failed because it was unable to allocate enough memory to perform the requested operation
    CudaOutofMemory = 39,
	/// This indicates that a resource handle passed to the API call was not valid
    ErrorNotReady = 40,
	/// Failed to create query for sample
    D3d11FailedToCreateQuery = 41,
	/// Generic OpenGL error, no real need to expose more detail since app will probably have an OpenGL error callback registered
    OpenglError = 42,
	/// Unknown error
    Unknown = 43,
}

fn remotery_error_to_string(id: RemoteryError) -> &'static str {
    match id {
        RemoteryError::Malloc => "Malloc call within remotery failed",
        RemoteryError::TlsAllocFail => "Attempt to allocate thread local storage failed",
        RemoteryError::VirtualMemoryBufferFail => "Failed to create a virtual memory mirror buffer",
        RemoteryError::CreateThreadFail => "Failed to create a thread for the server",

        RemoteryError::SocketInitNetworkFail => "Network initialisation failure (e.g. on Win32, WSAStartup fails)",
        RemoteryError::SocketCreateFail => "Can't create a socket for connection to the remote viewer",
        RemoteryError::SocketBindFail => "Can't bind a socket for the server",
        RemoteryError::SocketListenFail => "Created server socket failed to enter a listen state",
        RemoteryError::SocketSetNonBlockingFail => "Created server socket failed to switch to a non-blocking state",
        RemoteryError::SocketInvalidPoll => "Poll attempt on an invalid socket",
        RemoteryError::SocketSelectFail => "Server failed to call select on socket",
        RemoteryError::SocketPollErrors => "Poll notified that the socket has errors",
        RemoteryError::SocketAcceptFail => "Server failed to accept connection from client",
        RemoteryError::SocketSendTimeout => "Timed out trying to send data",
        RemoteryError::SocketSendFail => "Unrecoverable error occured while client/server tried to send data",
        RemoteryError::SocketRecvNoData => "No data available when attempting a receive",
        RemoteryError::SocketRecvTimeout => "Timed out trying to receive data",
        RemoteryError::SocketRecvFailed => "Unrecoverable error occured while client/server tried to receive data",

        RemoteryError::WebsocketHandshakeNotGet => "WebSocket server handshake failed, not HTTP GET",
        RemoteryError::WebsocketHandshakeNoVersion => "WebSocket server handshake failed, can't locate WebSocket version",
        RemoteryError::WebsocketHandshakeBadVersion => "WebSocket server handshake failed, unsupported WebSocket version",
        RemoteryError::WebsocketHandshakeNoHost => "WebSocket server handshake failed, can't locate host",
        RemoteryError::WebsocketHandshakeBadHost => "WebSocket server handshake failed, host is not allowed to connect",
        RemoteryError::WebsocketHandshakeNoKey => "WebSocket server handshake failed, can't locate WebSocket key",
        RemoteryError::WebsocketHandshakeBadKey => "WebSocket server handshake failed, WebSocket key is ill-formed",
        RemoteryError::WebsocketHandshakeStringFail => "WebSocket server handshake failed, internal error, bad string code",
        RemoteryError::WebsocketDisconnected => "WebSocket server received a disconnect request and closed the socket",
        RemoteryError::WebsocketBadFrameHeader => "Couldn't parse WebSocket frame header",
        RemoteryError::WebsocketBadFrameHeaderSize => "Partially received wide frame header size",
        RemoteryError::WebsocketBadFrameHeaderMask => "Partially received frame header data mask",
        RemoteryError::WebsocketReceiveTimeout => "Timeout receiving frame header",

        RemoteryError::RemoteryNotCreated => "Remotery object has not been created",
        RemoteryError::SendOnIncompleteProfile => "An attempt was made to send an incomplete profile tree to the client",
        RemoteryError::CudaDeinitialized => "This indicates that the CUDA driver is in the process of shutting down",
        RemoteryError::CudaNotInitialized => "This indicates that the CUDA driver has not been initialized with cuInit() or that initialization has failed",
        RemoteryError::CudaInvalidContext => "This most frequently indicates that there is no context bound to the current thread",
        RemoteryError::CudaInvalidValue => "This indicates that one or more of the parameters passed to the API call is not within an acceptable range of values",
        RemoteryError::CudaInvalidHandle => "This indicates that a resource handle passed to the API call was not valid",
        RemoteryError::CudaOutofMemory => "The API call failed because it was unable to allocate enough memory to perform the requested operation",
        RemoteryError::ErrorNotReady => "This indicates that a resource handle passed to the API call was not valid",
        RemoteryError::D3d11FailedToCreateQuery => "Failed to create query for sample",
        RemoteryError::OpenglError => "Generic OpenGL error, no real need to expose more detail since app will probably have an OpenGL error callback registered",
        RemoteryError::Unknown => "Unknown error",
    }
}

pub fn get_error(id: u32) -> RemoteryError {
    if id <= RemoteryError::Unknown as u32 {
        let error: RemoteryError = unsafe { ::std::mem::transmute(id as u8) };
        error
    } else {
        RemoteryError::Unknown
    }
}


impl fmt::Display for RemoteryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", remotery_error_to_string(*self))
    }
}

impl Error for RemoteryError  {
    fn description(&self) -> &str {
        remotery_error_to_string(*self)
    }
}
