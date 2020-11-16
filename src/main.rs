mod csvreader;
mod genlifetime;
mod iterate_lifetime;

use csvreader::{CsvLineParser, CsvReader};
use genlifetime::*;
use iterate_lifetime::*;

// main entry point
pub(crate) fn main() {
    let scores = vec![100, 189, 210, 209, 176];
    let wins: Vec<i32> = vec![4, 3, 2, 1];
    println!("{:?}", "do stuff");
    println!(
        "highest score is {:?} max wins {:?}",
        largest(&scores),
        largest(&wins)
    );

    let xpoint = Point { x: 1, y: 4 };
    println!("{:?}", xpoint);
    let ipoint = Point { x: 1, y: 2 };
    let fpoint = Point { x: 1.3, y: 2.7 };
    println!("{:?} {:?}", ipoint, fpoint);
    println!("{:?} {:?}", ipoint.get_x(), fpoint.get_x());
    let bpost = BlogPost {
        heading: String::from("heading"),
        date: String::from("10th"),
    };
    bpost.summarize();
    bpost.def_func();
    let a = String::from("hello");
    let result: &str;
    let b = String::from("byeeee");
    result = longest(&a, &b);
    println!("{}", result);

    parse_context(&genlifetime::Context(&String::from("echo")));

    let csv_reader = CsvReader::new(String::from(""));
    println!("{:?}", csv_reader);

    let csv_string = String::from("");
    let delim = String::from(",");
    let csv_line_parser = CsvLineParser::new(&csv_string, &delim);
    for (index, line) in csv_line_parser.enumerate() {
        println!("{:?} {:?}", index, line);
    }

    // for x in csv_reader {
    //     println!("{:?}", x);
    // }

    do_iterate();
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
}

impl Summarizable for BlogPost {
    fn summarize(&self) -> String {
        println!("{}", "data");
        format!("Blog Post - {:?}  posted on {:?}", self.heading, self.date)
    }

    fn fun() {}
}
