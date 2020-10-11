pub(crate) fn main() {
    let s = String::from("hello");
    println!("{}", s);
    let s2 = s.clone();
    println!("{}", s2);
    println!("{}", s);
    let vec = Vec::<i32>::new();
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec0", vec.len(), vec);
    vec1.push(23);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
