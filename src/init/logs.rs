use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::FormatTime;

struct LogsTime;

impl FormatTime for LogsTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        write!(w, "{}", now.format("%H:%M:%S"))
    }
}

pub fn init_logs() {
    let builder = EnvFilter::builder().with_default_directive(LevelFilter::WARN.into());

    let filter = match builder.from_env() {
        Ok(filter) => filter,
        Err(e) => {
            eprintln!("Invalid RUST_LOG env: {}", e);
            builder.from_env_lossy()
        }
    };

    let filter = filter.add_directive("novel_rs=trace".parse().expect("Invaled log directive"));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(filter)
        .with_target(false)
        .with_timer(LogsTime)
        .init();
}
