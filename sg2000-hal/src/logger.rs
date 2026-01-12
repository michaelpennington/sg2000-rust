use core::fmt::Write;
use heapless::{String, mpmc::Queue};
use log::{Level, LevelFilter, Metadata, Record, info};

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

/// Pop a log line from the queue.
/// Useful if you need custom handling of the log string.
pub fn pop_log() -> Option<String<LOG_LINE_LENGTH>> {
    LOG_QUEUE.dequeue()
}

pub struct RpmsgLogger<'a> {
    dev: RpmsgDevice<'a>,
}

impl<'a> RpmsgLogger<'a> {
    pub fn init() -> Self {
        let mut dev = unsafe { RpmsgDevice::new() };
        while !dev.is_driver_ok() {}
        let src_addr = 0x400;
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .unwrap();
        match dev.announce("rpmsg-tty", src_addr) {
            Ok(_) => info!("service 'rpmsg-tty' announced at {src_addr:#010X}"),
            Err(e) => info!("Announcement failed: {e}"),
        }

        info!("Hello from the remote core!");

        Self { dev }
    }

    pub fn flush_log(&mut self) {
        for _ in 0..LOG_QUEUE_DEPTH {
            if let Some(msg) = pop_log() {
                if self.dev.send(0x400, 0x400, msg.as_bytes()).is_err() {
                    break;
                }
            } else {
                break;
            }
        }
        self.dev.poll(|src, dst, data| {
            if let Ok(s) = core::str::from_utf8(data) {
                info!("RX [{}->{}]: {}", src, dst, s);
            } else {
                info!("RX [{}->{}]: {} bytes binary", src, dst, data.len());
            }
        });
    }
}
