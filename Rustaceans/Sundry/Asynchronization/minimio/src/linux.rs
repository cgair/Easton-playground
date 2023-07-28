use std::os::unix::io::{AsRawFd, RawFd};
use std::net::TcpStream;

use crate::{Interests, Token, Events};

pub struct Registrator {
    fd: RawFd,
}

impl Registrator {
    pub fn register(
        &self,
        stream: &TcpStream,
        token: Token,
        interest: Interests
    ) -> std::io::Result<()> {
        let fd = stream.as_raw_fd();
        if interest.is_readable() {
            // We register the id (or most oftenly referred to as a Token) to the `udata` field
            // if the `Kevent`
            let mut event = ffi::Event::new(ffi::EPOLLIN | ffi::EPOLLONESHOT, token);
            epoll_ctl(self.fd, ffi::EPOLL_CTL_ADD, fd, &mut event)?;
        }
        if interest.is_writable() {
            unimplemented!()
        }

        Ok(())
    }
}

// The selector is what's backing the Poll instance and is where the blocking call to wait for events occur.
#[derive(Debug)]
pub struct  Selector {
    fd: RawFd
}

impl Selector {
    pub fn new() -> std::io::Result<Self> {
        Ok(
            Selector {
                fd: epoll_create()?
            }
        )
    }

    /// This function blocks and waits until an event has been received. `timeout` None means
    /// the poll will never time out.
    pub fn select(&self, events: &mut Events, timeout_ms: Option<i32>) -> std::io::Result<()> {
        events.clear();
        let timeout = timeout_ms.unwrap_or(-1);
        epoll_wait(self.fd, events, 1024, timeout).map(|n_event| {
            unsafe { events.set_len(n_event as usize) };
        })
    }
}





pub type Event = ffi::Event;

fn epoll_create() -> std::io::Result<i32> {
    // Size argument is ignored but must be greater than zero
    let ret = unsafe { ffi::epoll_create(1) };
    return if ret < 0 { Err(std::io::Error::last_os_error()) } else { Ok(ret) } 
}

fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: &mut Event ) -> std::io::Result<()> {
    let ret = unsafe { ffi::epoll_ctl(epfd, op, fd, event) };
    return if ret < 0 { Err(std::io::Error::last_os_error()) } else { Ok(()) } 
}

/// Waits for events on the epoll instance to occur. Returns the number file descriptors ready for the requested I/O.
/// When successful, epoll_wait() returns the number of file descriptors ready for the requested
/// I/O, or zero if no file descriptor became ready during the requested timeout milliseconds
fn epoll_wait(epfd: i32, events: &mut [Event], maxevents: i32, timeout: i32) -> std::io::Result<i32> {
    let ret = unsafe { ffi::epoll_wait(epfd, events.as_mut_ptr(), maxevents, timeout) };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

fn close_fd(fd: i32) -> std::io::Result<i32> {
    let ret = unsafe { ffi::close(fd) };
    return if ret < 0 { Err(std::io::Error::last_os_error()) } else { Ok(ret) } 
}

fn eventfd(initva: u32, flags: i32) -> std::io::Result<i32> {
    let ret = unsafe { ffi::eventfd(initva, flags) };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

mod ffi {
    use std::io;
    use std::os::raw::c_void;

    pub const EPOLL_CTL_ADD: i32 = 1;
    pub const EPOLL_CTL_DEL: i32 = 2;
    pub const EPOLLIN: i32 = 0x1;
    pub const EPOLLONESHOT: i32 = 0x40000000;

    /// Since the same name is used multiple times, it can be confusing but we have an `Event` structure.
    /// This structure ties a file descriptor and a field called `events` together. The field `events` holds information
    /// about what events are ready for that file descriptor.
    #[repr(C, packed)]
    pub struct Event {
        /// This can be confusing, but this is the events that are ready on the file descriptor.
        events: u32,
        epoll_data: usize,
    }

    impl Event {
        pub fn new(events: i32, id: usize) -> Self {
            Event {
                events: events as u32,
                epoll_data: id,
            }
        }
        pub fn data(&self) -> usize {
            self.epoll_data
        }
    }

    #[link(name = "c")]
    extern "C" {
        /// http://man7.org/linux/man-pages/man2/epoll_create1.2.html
        pub fn epoll_create(size: i32) -> i32;

        /// http://man7.org/linux/man-pages/man2/close.2.html
        pub fn close(fd: i32) -> i32;

        /// http://man7.org/linux/man-pages/man2/epoll_ctl.2.html
        pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

        /// http://man7.org/linux/man-pages/man2/epoll_wait.2.html
        ///
        /// - epoll_event is a pointer to an array of Events
        /// - timeout of -1 means indefinite
        pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;

        /// http://man7.org/linux/man-pages/man2/timerfd_create.2.html
        pub fn eventfd(initva: u32, flags: i32) -> i32;
    }
}