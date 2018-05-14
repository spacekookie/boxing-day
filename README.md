# Boxing Day

This crate adds the `unwrap()` and `unwrap_mut()` functions to the `Box<T>` type that allows you to unwrap boxes of lovely presents. But only if it's boxing day!

Add `boxing-day` to your `Cargo.toml`

```
boxing-day = "0.1"
```

Then try this code out.

```
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
```
