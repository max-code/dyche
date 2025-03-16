#[macro_export]
macro_rules! log_call {
    ($command:expr, $ctx:expr) => {{
        use tracing::info;
        info!(
            "{} called by {} `{}`",
            $command,
            $ctx.author().id,
            $ctx.author().name
        );
    }};
    ($command:expr, $ctx:expr $(, $param_name:expr, $param_value:expr)*) => {{
        use tracing::info;
        info!(
            "{} called by {} `{}` with params: {}",
            $command,
            $ctx.author().id,
            $ctx.author().name,
            vec![$(format!("{}={:?}", $param_name, $param_value)),*].join(", ")
        );
    }};
}

#[macro_export]
macro_rules! start_timer {
    () => {{
        Instant::now()
    }};
}

#[macro_export]
macro_rules! log_timer {
    ($start:expr, $command:expr, $ctx:expr, $status:expr) => {{
        debug!(
            "{} called by {} {} after {:?}",
            $command,
            $ctx.author().id,
            $status,
            $start.elapsed()
        );
    }};
}
