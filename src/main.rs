pub(crate) fn main() {
    let scores = vec![100, 189, 210, 209, 176];
    let wins = vec![4, 3, 2, 1];
    println!(
        "highest score is {:?} max wins {:?}",
        largest(&scores),
        largest(&wins)
    );

    let ipoint = Point { x: 1, y: 2 };
    let fpoint = Point { x: 1.3, y: 2.7 };
    println!("{:?} {:?}", ipoint, fpoint);
    println!("{:?} {:?}", ipoint.get_x(), fpoint.get_x());
    let bpost = BlogPost {
        body: String::from("body"),
        heading: String::from("heading"),
        date: String::from("10th"),
    };
    bpost.summarize();
    bpost.def_func();
    let a = String::from("hello");
    {
        let b = String::from("byeeee");
        let result = longest(&a, &b);
        println!("{}", result);
    }
    let dao = Dao {
        name: &String::from("db"),
    };
    println!("{}", dao.fun());

    parse_context(&Context(&String::from("echo")));
}

struct Context<'a>(&'a str);

struct Parser<'a, 'c> {
    ctx: &'a Context<'c>,
}

impl<'a, 'c> Parser<'a, 'c> {
    fn echo(&self) -> &'c str {
        self.ctx.0
    }
}

fn parse_context<'a>(cntx: &Context<'a>) -> &'a str {
    Parser { ctx: cntx }.echo()
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

pub fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list {
        largest = if i > largest { i } else { largest }
    }
    return largest;
}

pub fn gen_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        largest = if i > largest { i } else { largest }
    }
    return largest;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

trait Summarizable {
    fn summarize(&self) -> String;
    fn fun();
    fn def_func(&self) {}
}

struct BlogPost {
    date: String,
    heading: String,
    body: String,
}

impl Summarizable for BlogPost {
    fn summarize(&self) -> String {
        format!("Blog Post - {:?}  posted on {:?}", self.heading, self.date)
    }

    fn fun() {}
}

struct Dao<'a> {
    name: &'a String,
}

impl<'a> Dao<'a> {
    fn fun(&self) -> &String {
        self.name
    }
}
