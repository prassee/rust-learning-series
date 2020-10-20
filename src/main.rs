pub(crate) fn main() {
    let a = String::from("hello");
    let b = String::from("world");
    longest(&a, &b);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
