// https://www.reddit.com/r/dailyprogrammer/comments/5196fi/20160905_challenge_282_easy_unusual_bases/

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate grabinput;

mod bits;
mod fib_u64;
mod sequence;

use fib_u64::Fib64;

fn main() {
    for line in grabinput::by_lines(std::env::args().nth(1)) {
        let mut parts = line.split_whitespace();

        match (parts.next(), parts.next()) {
            (Some("10"), Some(value)) => {
                match convert_decimal(value) {
                    None => println!("bad input: {}", value),
                    Some(value) => println!("{}", value),
                }
            }

            (Some("F"), Some(value)) => {
                match convert_fib(value) {
                    None => println!("bad input: {}", value),
                    Some(value) => println!("{}", value),
                }
            }

            _ => println!("bad input: {}", line),
        }
    }
}

#[inline]
fn convert_decimal(value: &str) -> Option<Fib64> {
    value.parse::<u64>().ok().map(|n| n.into())
}

#[inline]
fn convert_fib(value: &str) -> Option<u64> {
    value.parse::<Fib64>().ok().map(|n| n.into())
}
