use core::fmt::Write;
use heapless::{String, mpmc::Queue};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

#[cfg(feature = "log-rpmsg")]
use crate::rpmsg::RpmsgDevice;

const LOG_LINE_LENGTH: usize = 128;
const LOG_QUEUE_DEPTH: usize = 16;

#[expect(deprecated)]
static LOG_QUEUE: Queue<String<LOG_LINE_LENGTH>, LOG_QUEUE_DEPTH> = Queue::new();

struct GlobalLogger;

impl log::Log for GlobalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut s = String::<LOG_LINE_LENGTH>::new();
            if writeln!(&mut s, "[{}] {}", record.level(), record.args()).is_ok() {
                // If the queue is full, we drop the log to avoid blocking the system
                let _ = LOG_QUEUE.enqueue(s);
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: GlobalLogger = GlobalLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

/// Pop a log line from the queue.
/// Useful if you need custom handling of the log string.
pub fn pop_log() -> Option<String<LOG_LINE_LENGTH>> {
    LOG_QUEUE.dequeue()
}

/// Helper to drain the log queue directly into an RpmsgDevice.
/// Call this in your main loop.
#[cfg(feature = "log-rpmsg")]
pub fn drain_to_rpmsg(rpmsg: &mut RpmsgDevice, src: u32, dst: u32) {
    // We limit how many we process per tick to ensure we don't starve other tasks
    // if logs are being generated faster than we can send them.
    for _ in 0..LOG_QUEUE_DEPTH {
        if let Some(msg) = pop_log() {
            if rpmsg.send(src, dst, msg.as_bytes()).is_err() {
                // If TX ring is full, we stop draining for now and try again later.
                // Note: The message we just popped is dropped here.
                break;
            }
        } else {
            break;
        }
    }
}
