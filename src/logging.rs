use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

// pub fn init_tracing() {
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())
//         .with_thread_names(true)
//         .init();
// }

static mut GUARD: Option<tracing_appender::non_blocking::WorkerGuard> = None;

/// 初始化 tracing，并写入 logs/myapp.log（每日滚动）
pub fn init_tracing_to_file() {
    let file_appender = rolling::daily("logs", "myapp.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 保留 guard，防止日志线程提前退出
    unsafe {
        GUARD = Some(guard);
    }

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            // 支持 RUST_LOG 也有默认等级
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .init();
}
