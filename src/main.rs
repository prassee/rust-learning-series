pub(crate) fn main() {
    let red = Color(1, 0, 0);
    let dark_red = Color(red.0 + 1, 0, 0);
    println!("{:?} {:?}", red, dark_red);
    // let dir = Directions::North;
    let friend_loc = RelLocation {
        from: String::from(""),
        direction: Directions::South,
    };
    let area = friend_loc.dir_as_str();
    println!("{:?}", area);
    let area2 = friend_loc.dir_as_str();
    println!("{:?}", area2);

    let new_dir = RelLocation::new(String::from(""), Directions::South);
    println!("{:?}", new_dir);
    println!("{:?}", friend_loc);
    log(String::from(""), LogLevel::Trace);
    let tsbt = Some(true);
    let tsbn: Option<bool> = None;
    match tsbn {
        Some(t) => println!("{}", t),
        None => println!("false"),
    }
    println!("{:?} {:?}", tsbt, tsbn);

    println!(
        "with none {:?} with some {:?}",
        has_value(None),
        has_value(Some(23))
    );
}

#[derive(Debug)]
struct Point(i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
#[allow(dead_code)]
enum Directions {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct RelLocation {
    from: String,
    direction: Directions,
}

impl RelLocation {
    fn dir_as_str(&self) -> i32 {
        println!("{:?}", self.from);
        return 23;
    }

    fn new(from: String, dir: Directions) -> RelLocation {
        RelLocation {
            from,
            direction: dir,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum LogLevel {
    Info = 0,
    Warning = 1,
    Error = 2,
    Trace = 3,
}

#[allow(dead_code)]
impl LogLevel {
    fn info(&self, line: String) {
        println!("{}", line);
    }

    fn update(&self) {}
}

fn log(line: String, level: LogLevel) {
    match level as i32 {
        0 => println!("info"),
        1 => println!("warning"),
        2 => println!("error"),
        3 => println!("trace"),
        _ => println!("trace"),
    }
    println!("{}", line);
}

fn has_value(a: Option<i32>) -> bool {
    return match a {
        None => false,
        Some(_) => true,
    };
}
