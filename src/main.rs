pub(crate) fn main() {
    let mut v = Vec::new();
    v.push(23);
    v.push(21);
    v.push(21);
    println!("{:?}", v);

    let vect = vec![23, 22, 21];
    println!("{:?}", vect);

    println!("first value {:?} second value {:?}", vect[0], vect.get(2));

    for i in &vect {
        println!("{:?}", i);
    }

    println!("first value {:?} second value {:?}", vect[0], vect.get(2));

    let mut vect2 = vec![23, 22, 21];

    for i in &mut vect2 {
        *i += 2
    }
    println!("{:?}", vect2);

    let x = 23;
    println!("{}", x);
}
