// we'll implement the [minimiolibrary](https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/the-recipie-for-an-eventqueue), an extremely simplified version of a cross platform event queue. 
/**
 * The [parts](https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/the-recipie-for-an-eventqueue#the-parts-we-need-to-create) we need to create.
 * Requirements
 * We want to be able to block the current thread while we wait for events;
 * We want to be able to register interest from a different thread than we run the main loop on;
 */

mod linux;
pub use linux::Event;

const WRITABLE: u8 = 0b0000_0001;
const READABLE: u8 = 0b0000_0010;

pub type Events = Vec<Event>;
pub type Token = usize;

/// `Poll` represents the event queue. The `poll` method will block the current thread
/// waiting for events. If no timeout is provided it will potentially block indefinately.
/// 
/// `Poll` can be used in one of two ways. The first way is by registering interest in events and then wait for
/// them in the same thread. In this case you'll use the built-in methods on `Poll` for registering events.
/// 
/// Alternatively, it can be used by waiting in one thread and registering interest in events from
/// another. In this case you'll need to call the `Poll::registrator()` method which returns a `Registrator`
/// tied to this event queue which can be sent to another thread and used to register events.
#[derive(Debug)]
pub struct Poll {}

impl Poll {

}

#[derive(Debug)]
pub struct Registry {}

/// Represents interest in either Read or Write events. This struct is created 
/// by using one of the two constants:
/// 
/// - Interests::READABLE
/// - Interests::WRITABLE
pub struct Interests(u8);

impl Interests {
    pub const READABLE: Interests = Interests(READABLE);
    pub const WRITABLE: Interests = Interests(WRITABLE);

    pub fn is_readable(&self) -> bool {
        self.0 & READABLE != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & WRITABLE != 0
    }
}