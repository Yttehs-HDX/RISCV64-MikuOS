use color_filter::ColorFilter;
use log::{debug, Log};

mod color_filter;

static LOGGER: Logger = Logger;

pub fn logger_init() {
    use log::LevelFilter;
    log::set_logger(&LOGGER).unwrap();
    let max_level = match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    log::set_max_level(max_level);

    debug!("[Logger] initialized");
    debug!("[Logger] level: {}", max_level);
}

// region Logger begin
struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let color_filter = ColorFilter::from(record.level());
            println!(
                "{}[{}] {}{}",
                color_filter.color(),
                record.level(),
                record.args(),
                ColorFilter::Reset.color(),
            );
        }
    }

    fn flush(&self) {
    }
}
// region Logger end