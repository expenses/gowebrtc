pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::os::raw::c_char;
use std::ffi::{CStr, c_void};
use std::io;

pub struct Transport(pub i64);

impl Transport {
    pub fn new() -> Self {
        let id = unsafe {
            bindings::transport_new()
        };

        Self(id)
    }

    pub fn listen(&self, multiaddr: parity_multiaddr::Multiaddr) -> io::Result<Listener> {
        let multiaddr_string = multiaddr.to_string();
        let multiaddr = go_string(&multiaddr_string);
        
        let result = unsafe {
            bindings::transport_listen(self.0, multiaddr)
        };

        match result.r0 {
            -1 => Err(ptr_to_io_error(result.r1)),
            id => Ok(Listener(id))
        }
    }

    pub fn dial(&self, multiaddr: parity_multiaddr::Multiaddr, peer_id: &str) -> io::Result<Connection> {
        let multiaddr_string = multiaddr.to_string();
        let multiaddr = go_string(&multiaddr_string);
        let peer_id = go_string(&peer_id);

        let result = unsafe {
            bindings::transport_dial(self.0, multiaddr, peer_id)
        };

        match result.r0 {
            -1 => Err(ptr_to_io_error(result.r1)),
            id => Ok(Connection(id))
        }
    }
}


pub struct Listener(pub i64);

impl Listener {
    pub fn accept(&self) -> io::Result<Connection> {
        let result = unsafe {
            bindings::listener_accept(self.0)
        };

        match result.r0 {
            -1 => Err(ptr_to_io_error(result.r1)),
            id => Ok(Connection(id))
        }
    }

    pub fn multiaddr(&self) -> parity_multiaddr::Multiaddr {
        let pointer = unsafe {
            bindings::listener_multiaddr(self.0)
        };

        let string = unsafe {
            CStr::from_ptr(pointer)
        }.to_str().unwrap();

        string.parse().unwrap()
    }
}

#[derive(Debug)]
pub struct Connection(pub i64);

impl Connection {
    pub fn accept_stream(&self) -> io::Result<Stream> {
        let result = unsafe {
            bindings::connection_accept_stream(self.0)
        };

        match result.r0 {
            -1 => Err(ptr_to_io_error(result.r1)),
            id => Ok(Stream(id))
        }
    }

    pub fn open_stream(&self) -> io::Result<Stream> {
        let result = unsafe {
            bindings::connection_open_stream(self.0)
        };

        match result.r0 {
            -1 => Err(ptr_to_io_error(result.r1)),
            id => Ok(Stream(id))
        }
    }

    pub fn local_multiaddr(&self) -> parity_multiaddr::Multiaddr {
        let pointer = unsafe {
            bindings::connection_local_multiaddr(self.0)
        };

        let string = unsafe {
            CStr::from_ptr(pointer)
        }.to_str().unwrap();

        string.parse().unwrap()
    }

    pub fn remote_multiaddr(&self) -> parity_multiaddr::Multiaddr {
        let pointer = unsafe {
            bindings::connection_remote_multiaddr(self.0)
        };

        let string = unsafe {
            CStr::from_ptr(pointer)
        }.to_str().unwrap();

        string.parse().unwrap()
    }
}

pub struct Stream(pub i64);

impl std::io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let slice = go_slice(buf);

        let result = unsafe {
            bindings::stream_read(self.0, slice)
        };

        if !result.r1.is_null() {
            Err(ptr_to_io_error(result.r1))
        } else {
            Ok(result.r0 as usize)
        }
    }
}

impl std::io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let slice = go_slice(buf);

        let result = unsafe {
            bindings::stream_write(self.0, slice)
        };

        if !result.r1.is_null() {
            Err(ptr_to_io_error(result.r1))
        } else {
            Ok(result.r0 as usize)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}



impl Drop for Stream {
    fn drop(&mut self) {
        let error = unsafe {
            bindings::stream_close(self.0)
        };

        if !error.is_null() {
            println!("{}", ptr_to_io_error(error))
        }
    }
}

fn go_slice(buf: &[u8]) -> bindings::GoSlice {
    bindings::GoSlice {
        data: buf.as_ptr() as *mut c_void,
        len: buf.len() as i64,
        cap: buf.len() as i64
    }
}

fn go_string(string: &str) -> bindings::GoString {
    bindings::GoString {
        p: string.as_ptr() as *const c_char,
        n: string.len() as isize
    }
}

fn ptr_to_io_error(pointer: *const c_char) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        unsafe {
            CStr::from_ptr(pointer)
        }.to_str().unwrap()
    )
}
