use core::fmt::Write;
use heapless::{String, mpmc::Queue};
use log::{Level, LevelFilter, SetLoggerError};

const LOG_LINE_LENGTH: usize = 128;
const LOG_QUEUE_DEPTH: usize = 16;

#[expect(deprecated)]
static LOG_QUEUE: Queue<String<LOG_LINE_LENGTH>, LOG_QUEUE_DEPTH> = Queue::new();

struct RpmsgLogger;

impl log::Log for RpmsgLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut s = String::<LOG_LINE_LENGTH>::new();

            if writeln!(&mut s, "[{}] {}", record.level(), record.args()).is_ok() {
                let _ = LOG_QUEUE.enqueue(s);
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: RpmsgLogger = RpmsgLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub fn pop_log() -> Option<String<LOG_LINE_LENGTH>> {
    LOG_QUEUE.dequeue()
}
