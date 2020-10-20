extern crate rust_yt_series;

use rust_yt_series::{api, publisher};

pub(crate) fn main() {
    publisher::connect();
    api::api_call();
    rust_yt_series::publisher::connect();
    rust_yt_series::api::api_call();
}
