mod logger;

pub fn api_call() {
    local_log();
}

fn local_log() {
    logger::log_api_call();
}
