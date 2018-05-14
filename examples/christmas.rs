extern crate boxing_day;
use boxing_day::Unwrappable;
use std::fmt;

struct Present<'a> {
    name: &'a str,
}

impl<'a> fmt::Display for Present<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    let present = Box::new(Present {
        name: "Amazon Coupon",
    });

    println!(
        "On this day, you received: {}",
        match present.unwrap() {
            Some(p) => format!("{}", p),
            None => format!("...NOTHING! Because it's not actually boxing day!"),
        }
    );
}
