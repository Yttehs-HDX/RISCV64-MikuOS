use log::Level;

// region ColorFilter begin
pub enum ColorFilter {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Reset,
}

impl From<Level> for ColorFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => ColorFilter::Error,
            Level::Warn => ColorFilter::Warn,
            Level::Info => ColorFilter::Info,
            Level::Debug => ColorFilter::Debug,
            Level::Trace => ColorFilter::Trace,
        }
    }
}

impl ColorFilter {
    pub fn color(&self) -> &str {
        match self {
            ColorFilter::Error => "\x1b[31m",
            ColorFilter::Warn => "\x1b[33m",
            ColorFilter::Info => "\x1b[32m",
            ColorFilter::Debug => "\x1b[34m",
            ColorFilter::Trace => "\x1b[35m",
            ColorFilter::Reset => "\x1b[0m",
        }
    }
}
// region ColorFilter end