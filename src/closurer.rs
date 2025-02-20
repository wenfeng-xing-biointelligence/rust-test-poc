use crate::provider::{self, Provider};

pub fn closurer(provider: Box<dyn Provider>) -> Box<dyn Fn() -> i32>{
    Box::new(move || provider.get())
}

pub fn adder(provider: Box<dyn Provider>) -> Box<dyn Fn(i32) -> i32>{
    Box::new(move |x| x + provider.get())
}