extern crate chrono;

use chrono::{Date, Datelike, Local};
use std::boxed::Box;

pub trait Unwrappable<T> {
    /// Unwrap this box and get &T if you're allowed to
    fn unwrap(&self) -> Option<&T>;
    /// Unwrap this box and get &mut T if you're allowed to
    fn unwrap_mut(&mut self) -> Option<&mut T>;
}

impl<T> Unwrappable<T> for Box<T> {
    fn unwrap(&self) -> Option<&T> {
        let dt: Date<Local> = Local::today();
        match (&dt.day(), &dt.month()) {
            (26, 12) => Some(self),
            _ => None,
        }
    }

    fn unwrap_mut(&mut self) -> Option<&mut T> {
        let dt: Date<Local> = Local::today();
        match (&dt.day(), &dt.month()) {
            (26, 12) => Some(self),
            _ => None,
        }
    }
}
