use bits::Fiddle;
use sequence;
use std::fmt;
use std::str;

#[derive(Copy, Clone, Debug)]
pub struct Fib64(pub u64);

#[derive(Debug)]
pub enum ParseFib64Error {
    Empty,
    InvalidDigit,
}

impl str::FromStr for Fib64 {
    type Err = ParseFib64Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseFib64Error::Empty);
        }

        let mut n = Fib64(0);
        for (idx, c) in s.chars().rev().enumerate() {
            match c {
                '1' => n = Fib64(n.set(idx as u64)),
                '0' => (),
                _ => return Err(ParseFib64Error::InvalidDigit),
            }
        }

        Ok(n)
    }
}

impl fmt::Display for Fib64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::with_capacity(32);
        for bit in self.bits().skip_while(|bit| !bit) {
            if bit { buf.push('1') } else { buf.push('0') }
        }

        f.write_str(&buf)
    }
}

impl From<u64> for Fib64 {
    fn from(value: u64) -> Self {
        let mut n = value;
        Fib64(sequence::SEQ.iter().enumerate().fold(0u64, |mut a, (idx, &fib)| {
            if fib > n || n == 0 {
                return a;
            }

            n -= fib;
            a = a.set(63 - idx as u64);
            a
        }))
    }
}

impl From<Fib64> for u64 {
    fn from(value: Fib64) -> Self {
        value.bits().enumerate().fold(0u64, |mut a, (idx, is_set)| if is_set {
            a += sequence::SEQ[idx];
            a
        } else {
            a
        })
    }
}

#[cfg(test)]
mod tests {
    use fib_u64::Fib64;

    #[test]
    fn from_u64_works() {
        let test_cases = &[(16, "1001000"),
                           (19, "1010010"),
                           (32, "10101000"),
                           (9024720, "1010100101010100000010001000010010")];

        for &(input, conversion) in test_cases {
            let result: Fib64 = input.into();
            assert_eq!(conversion, result.to_string());
        }
    }

    #[test]
    fn from_str_works() {
        let test_cases: &[(u64, &str)] = &[(16, "1001000"),
                                           (19, "1010010"),
                                           (32, "10101000"),
                                           (9024720, "1010100101010100000010001000010010")];

        for &(conversion, input) in test_cases {
            let result = input.parse::<Fib64>()
                .expect(&format!("unable to parse Fib64: {:?}", input));

            assert_eq!(conversion, result.into(), "failed to convert: {}", input);
        }
    }
}
