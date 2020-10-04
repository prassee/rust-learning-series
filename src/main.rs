pub(crate) fn main() {
    // mutate by shadowing
    let x = 90;
    println!("{}", x);
    // x = x + 9; // error

    let x = 90;
    println!("{}", x);

    let mut y = 90.0;
    println!("{}", y);
    y = 90.34;
    println!("{}", y);

    let my_name = "ping";
    println!("{}", my_name);
    println!("{}", my_name);

    // to avoid above use constant
    const YOB: i32 = 1984;
    println!("const yob {}", YOB);
    // const YOB: i32 = 1983; this line throw error

    // video notes

    /*
    var canbe signed or unsigned
    f64 is default
    bool for  T / F
    mut key word to define mutable valiables
    uses ASCII be default
    */

    let age: u8 = 34;
    let height = 5.7; // by default its f64
    let mut isWFH = true;
    isWFH = false;
    /*
    compund types
    array
    starts with []
    rust has type inference
    when defininging array type cand len defined by [type;len]

    tuples
    starts with ()
    can be made mut
    can be deconstructed by assigning to variables
    a tup can be nested in another
    */

    let scores = [129, 234, 157, 217];
    let mut strike_rates: [f64; 4] = [107.9, 200.0, 178.7, 183.0];
    for sr in strike_rates.iter() {
        println!("{}", sr);
    }
    strike_rates[0] = 207.9;

    let inn1 = (207, 8, 10.7);
    let inn2 = (205, 8, 10.5);
    let match_score = (inn1, inn2);
    println!("{:?}", match_score);
    let inn1RR = match_score.0.2;
    let mut bowling = (1,5,0,0,5.0,0);
    bowling.0 = bowling.0 + 1;
    bowling.1 = bowling.1 + 10;
    bowling.2 = bowling.2 + 0;
    bowling.3 = bowling.3 + 0;
    bowling.4 = bowling.4 + 0.0;
    bowling.5 = bowling.5 + 0;

    /*
        in rust 
        stmt ends with ; 
        expressions dont end with ; 
        or have a return
    */
}
