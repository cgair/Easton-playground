// [`Epoll` - the express version](https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/part-1-an-express-explanation/the-basics)

mod ffi {
    pub const EPOLL_CTL_ADD: i32 = 1;
    pub const EPOLLIN: i32 = 0x1;
    pub const EPOLLONESHOT: i32 = 0x40000000;

    #[link(name = "c")]
    extern "C" {
        pub fn epoll_create(size: i32) -> i32;
        pub fn epoll_ctl(epfd: i32, op: i32, fs: i32, event: *mut Event) -> i32;
        pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
        pub fn close(fd: i32) -> i32;
    }

    // #[repr(C)]
    // pub union Data {
    //     void: *const std::os::raw::c_void,
    //     fd: i32,
    //     uint32: u32,
    //     uint64: u64
    // }

    // #[repr(C)]
    // pub struct Event {
    //     event: u32,
    //     epoll_data: Data
    // }

    // Since the union defined both u32 and u64 as valid data, we can just use a usize, which should work.
    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct Event {
        pub(crate) events: u32,
        pub(crate) epoll_data: usize,
    }
}

use std::net::TcpStream;
use std::io::Write;
use std::os::unix::io::AsRawFd;

fn main() {
    // A counter to keep track of how many events we're expecting to act on
    let mut event_counter = 0;

    // First we create the event queue.
    // The size argument is ignored but needs to be larger than 0
    let queue = unsafe { ffi::epoll_create(1) };

    // This is how we basically check for errors and handle them using most
    // C APIs
    // We handle them by just panicking here in our example.
    if queue < 0 {
        panic!("[-] {}", std::io::Error::last_os_error());
    }

    // As you'll see below, we need a place to store the streams so they're
    // not closed
    let mut streams = vec![];
    // We crate 5 requests to an endpoint we control the delay on
    for x in 1..6 {
        // This site has an API to simulate slow responses from a server
        let addr = "flash.siwalik.in:80";
        let mut stream = TcpStream::connect(addr).unwrap();

        // The delay is passed in to the GET request as milliseconds.
        // We'll create delays in descending order so we should receive
        // them as `5, 4, 3, 2, 1`
        let delay = (5 - x) * 1000;

        let request = format!(
            "GET /delay/{}/url/http://www.google.com HTTP/1.1\r\n\
             Host: flash.siwalik.in\r\n\
             Connection: close\r\n\
             \r\n",
            delay
        );
        stream.write_all(request.as_bytes()).unwrap();
        // stream.set_nonblocking(true).unwrap();

        // Then register interest in getting notified for `Read` events on this socket. 
        // The `Event` struct is where we specify what events we want to register interest in and other configurations using flags.
        // `EPOLLIN` is interest in `Read` events.
        // `EPOLLONESHOT` means that we remove any interests from the queue after first event. 
        // If we don't do that we need to `deregister` our interest manually when we're done with the socket.
        // `epoll_data` is user provided data, so we can put a pointer or an integer value there to identify the event. 
        // We just use `i` which is the loop count to identify the events.
        let mut event = ffi::Event {
            events: (ffi::EPOLLIN | ffi::EPOLLONESHOT) as u32,
            epoll_data: x,
        };
        // This is the call where we actually `ADD` an interest to our queue.
        // `EPOLL_CTL_ADD` is the flag which controls whether we want to add interest, 
        // modify an existing one or remove interests from the queue.
        let op = ffi::EPOLL_CTL_ADD;
        let ret = unsafe {
            ffi::epoll_ctl(queue, op, stream.as_raw_fd(), &mut event)
        };
        if ret < 0 { panic!("[-] {}", std::io::Error::last_os_error()); } 
        // Letting `stream` go out of scope in Rust automatically runs its destructor which closes the socket. 
        // We prevent that by holding on to it until we're finished
        streams.push(stream);
        event_counter += 1;

    };

    // Now we wait for events
    while event_counter > 0 {
        // The API expects us to pass in an arary of `Event` structs.
        // This is how the OS communicates back to us what has happened.
        let mut events = Vec::with_capacity(10);
        // This call will actually block until an event occurs. The timeout of `-1` means no timeout so we'll block until something happens.
        // Now the OS suspends our thread doing a context switch and works on something else - or just preserves power.
        let ret = unsafe { ffi::epoll_wait(queue, events.as_mut_ptr(), 10, -1 as i32) };
        // This result will return the number of events which occurred (if any) or a negative number in case of an error.
        if ret < 0 { panic!("[-] {}", std::io::Error::last_os_error()); }
        // This one unsafe we could avoid though but this technique is used in libraries like `mio` and is safe as long as the OS does what it's supposed to.
        unsafe { events.set_len(ret as usize) };

        for event in events {
            println!("[+] RECEIVED: {:?}", event);
            event_counter -= 1;
        }
    }
    // When we manually initialize resources we need to manually clean up after our selves as well. Normally, in Rust, there will be a `Drop` implementation which takes care of this for us.
    let ret = unsafe { ffi::close(queue) };
    if ret < 0 { panic!("[-] {}", std::io::Error::last_os_error()); }
    println!("[+] FINISHED");
}