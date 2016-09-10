use fib_u64::Fib64;
use std::iter::Rev;
use std::ops::Range;

pub trait Fiddle {
    fn set(self, bit: u64) -> u64;
    fn unset(self, bit: u64) -> u64;
    fn bits(self) -> Bits;
}

impl Fiddle for u64 {
    fn set(self, bit: u64) -> u64 {
        set(self, bit)
    }

    fn unset(self, bit: u64) -> u64 {
        unset(self, bit)
    }

    fn bits(self) -> Bits {
        Bits::new(self)
    }
}

impl Fiddle for Fib64 {
    fn set(self, bit: u64) -> u64 {
        set(self.0, bit)
    }

    fn unset(self, bit: u64) -> u64 {
        unset(self.0, bit)
    }

    fn bits(self) -> Bits {
        Bits::new(self.0)
    }
}

#[derive(Clone)]
pub struct Bits {
    value: u64,
    idx: Rev<Range<u64>>,
}

impl Bits {
    fn new(value: u64) -> Bits {
        Bits {
            value: value,
            idx: (0..64).rev(),
        }
    }
}

impl Iterator for Bits {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx.next().map(|idx| check(self.value, idx))
    }
}

fn set(mut n: u64, bit: u64) -> u64 {
    n |= 1 << bit;
    n
}

fn unset(mut n: u64, bit: u64) -> u64 {
    n &= !(1 << bit);
    n
}

fn check(n: u64, bit: u64) -> bool {
    1 == ((n >> bit) & 1)
}

#[cfg(test)]
mod tests {
    use super::Fiddle;

    #[test]
    fn set_works() {

        let test_cases =
            &[(vec![0], 1, 0), (vec![1], 2, 0), (vec![0, 1], 3, 0), (vec![0, 1], 3, 3)];

        for &(ref set_these, expected_result, initial_value) in test_cases {
            let mut test_val = initial_value;

            for &bit in set_these {
                test_val = test_val.set(bit);
            }

            println!("{}", test_val);
            assert_eq!(test_val, expected_result);
        }
    }

    #[test]
    fn unset_works() {
        let test_cases = &[(vec![0], 1), (vec![1], 2), (vec![0, 1], 3), (vec![0, 1], 0)];

        for &(ref unset_these, initial_value) in test_cases {
            let mut test_val = initial_value;

            for &bit in unset_these {
                test_val = test_val.unset(bit);
            }

            println!("{}", test_val);
            assert_eq!(test_val, 0, "initial value: {}", initial_value);
        }
    }

    #[test]
    fn bit_iterator_works() {
        assert!(u64::max_value().bits().all(|bit| bit == true));
        assert!(u64::min_value().bits().all(|bit| bit == false));
    }
}
