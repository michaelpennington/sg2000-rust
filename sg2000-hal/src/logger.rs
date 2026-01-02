use core::{
    fmt::Write,
    slice,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::{Log, SetLoggerError};

use crate::resource_table::{TRACE_BUFFER_DA, TRACE_BUFFER_SIZE};

static WRITE_INDEX: AtomicUsize = AtomicUsize::new(0);

struct SharedLogger;

impl Log for SharedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            riscv::interrupt::free(|| {
                let mut writer = TraceWriter::new();

                let _ = writeln!(writer, "[{}] {}", record.level(), record.args());
            });
        }
    }

    fn flush(&self) {
        unsafe { xuantie_riscv::asm::sync() };
        riscv::asm::fence();
    }
}

struct TraceWriter;

impl TraceWriter {
    const fn new() -> Self {
        Self
    }
}

impl Write for TraceWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut id = WRITE_INDEX.load(Ordering::Relaxed);

        let trace_buffer =
            unsafe { slice::from_raw_parts_mut(TRACE_BUFFER_DA as *mut u8, TRACE_BUFFER_SIZE) };
        let mut it = s
            .bytes()
            .zip((0..TRACE_BUFFER_SIZE).cycle().skip(id))
            .peekable();
        while let Some((byte, idx)) = it.next() {
            trace_buffer[idx] = byte;
            if it.peek().is_none() {
                id = idx;
            }
        }

        WRITE_INDEX.store(id, Ordering::Relaxed);

        Ok(())
    }
}

static LOGGER: SharedLogger = SharedLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
}
