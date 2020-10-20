extern crate rust_yt_series;

pub(crate) fn main() {
    rust_yt_series::publisher::connect();
    rust_yt_series::api::api_call();
}
