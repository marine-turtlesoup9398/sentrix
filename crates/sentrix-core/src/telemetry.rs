use tracing_subscriber::{fmt, EnvFilter};

pub fn init_telemetry(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("sentrix=debug,info")
    } else {
        EnvFilter::new("sentrix=info,error")
    };

    let _ = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
