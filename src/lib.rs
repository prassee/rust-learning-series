pub mod api;
mod subscriber;
mod util;

// in-line module
pub mod publisher {

    use crate::util;
    pub fn connect() {
        util::logger(String::from("logged by publisher"));
    }
}
