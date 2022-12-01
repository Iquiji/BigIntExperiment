use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Sub, Mul, Shl};
use std::{fmt::Display, ops::Add};
use rand;

fn main() {
    println!("Hello, world!");
    let number = BigInt::<4>::from_hex_string("FF_00_00_FF");
    let number2 = BigInt::<4>::from_hex_string("00_00_00_01");
    println!("Debug: {:?}", number);
    println!("Debug: {:?}", number.shl(1));
    println!("Debug: {:?}", number2);
    println!("Debug: {:?}", number2.shl(1));
    println!("Debug: {:?}", number2.shl(2));
    println!("Debug: {:?}", number2.shl(3));
    println!("Debug: {:?}", number2.shl(4));
    println!("Debug: {:?}", number2.shl(5));
    println!("Debug: {:?}", number2.shl(6));
    println!("Debug: {:?}", number2.shl(7));
    println!("Debug: {:?}", number2.shl(8).shr(8));
    println!("Debug: {:?}", number2.shl(9).shr(9));
    println!("Debug: {:?}", number.shl(16).shr(16));
    // let double = number + number2;
    // println!("Debug: {:?}", double);
    // let number3 = number - number2;
    // println!("Debug: {:?}", number3);

}

#[derive(Clone, Copy, PartialEq, Eq)]
struct BigInt<const SIZE: usize> {
    data: [u8; SIZE],
}
impl<const SIZE: usize> BigInt<{ SIZE }> {
    fn new() -> Self {
        BigInt { data: [0; SIZE] }
    }

    fn from_u8(num: u8) -> Self{
        let mut int = Self::new();
        int.data[0] = num;
        int
    }

    fn from_hex_string(string: &str) -> Self {
        // println!("parse...");
        let mut data = [0; SIZE];
        let str_filtered = string.chars().filter(char::is_ascii_alphanumeric).collect::<String>();
        // println!("string filtered: '{}'",str_filtered);
        let str_iter = str_filtered.chars();
        for idx in (0..SIZE).rev() {
            let hex_4 = str_iter.clone().skip(SIZE * 2 - 2 * idx - 2).take(2).collect::<String>();
            // println!("hex_4: {hex_4}");
            let parsed =
                u8::from_str_radix(&hex_4, 16).expect("Error Parsing Hex String into BigInt");

            data[idx] = parsed;
        }
        BigInt { data }
    }
}

impl<const SIZE: usize> Add for BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut carry_flag = false;
        let mut output = BigInt::<SIZE>::new();

        for idx in 0..SIZE {
            // println!("carry flag: {}", carry_flag);
            let step = self.data[idx].checked_add(rhs.data[idx]);
            if let Some(step) = step {
                output.data[idx] =
                    if let Some(step_inner) = step.checked_add(if carry_flag { 1 } else { 0 }) {
                        carry_flag = false;
                        step_inner
                    } else {
                        carry_flag = true;
                        step.wrapping_add(1)
                    };
            } else {
                // println!("overflow in add");
                output.data[idx] = self.data[idx].wrapping_add(rhs.data[idx]);

                carry_flag = true;
            }
        }

        eprintln!("carry at end of add: {}", carry_flag);

        output
    }
}

impl<const SIZE: usize> Sub for BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut carry_flag = false;
        let mut output = BigInt::<SIZE>::new();

        for idx in 0..SIZE {
            // println!("carry flag: {}", carry_flag);
            let step = self.data[idx].checked_sub(rhs.data[idx]);
            println!("{} - {} = {:?}",self.data[idx],rhs.data[idx], step);
            if let Some(step) = step {
                output.data[idx] =
                    if let Some(step_inner) = step.checked_sub(if carry_flag { 1 } else { 0 }) {
                        carry_flag = false;
                        step_inner
                    } else {
                        carry_flag = true;
                        step.wrapping_sub(1)
                    };
            } else {
                println!("underflow in sub");
                output.data[idx] = self.data[idx].wrapping_sub(rhs.data[idx]) - if carry_flag { 1 } else { 0 };

                carry_flag = true;
            }
        }

        eprintln!("carry at end of sub: {}", carry_flag);

        output
    }
}

impl<const SIZE: usize> Mul for BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut carry_flag = false;
        let output = BigInt::new();
        let mut one = BigInt::<SIZE>::new();
        one.data[0] = one.data[0] + 1;
        let mut down_counter = rhs;
        // while !(down_counter == 0){

        // }

        output
    }
}

impl<const SIZE: usize> Ord for BigInt<{ SIZE }>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        for idx in (0..SIZE).rev(){
            if self.data[idx] > other.data[idx]{
                return std::cmp::Ordering::Greater;
            }
            if self.data[idx] < other.data[idx]{
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl<const SIZE: usize> PartialOrd for BigInt<{ SIZE }> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl<const SIZE: usize> BigInt<{ SIZE }> {
    fn shl_once(self) -> Self{
        let mut output = [0; SIZE];
        let mut carry = false;
        for idx in 0..SIZE{
            output[idx] = (self.data[idx] << 1) + if carry {1} else {0};
            // overflow 
            if (self.data[idx] << 1 ) < self.data[idx]{
                carry = true;
            }
            else{
                carry = false;
            }
        }

        BigInt { data: output }
    }

    fn shl(self, amount: usize) -> Self{
        let mut output = self;
        for _ in 0..amount{
            output = output.shl_once();
        }
        output
    }

    fn shr_once(self) -> Self{
        let mut output = [0; SIZE];
        let mut carry = false;
        for idx in (0..SIZE).rev(){
            output[idx] = (self.data[idx] >> 1) + if carry {0b1000_0000} else {0};
            // overflow 
            if (self.data[idx] & 1 ) == 1{
                carry = true;
            }
            else{
                carry = false;
            }
        }

        BigInt { data: output }
    }

    fn shr(self, amount: usize) -> Self{
        let mut output = self;
        for _ in 0..amount{
            output = output.shr_once();
        }
        output
    }
}

impl<const SIZE: usize> Display for BigInt<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<const SIZE: usize> Debug for BigInt<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = self.data;
        data.reverse();
        write!(f, "{:?}", data)
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_32bit_sub(){
        use crate::BigInt;

        let cases = [(1u32, 1u32), (0, 0), (u32::MAX, 1), (10000, 1234)];
        for case in cases{
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt{ data: number1_bytes};
            println!("number1 : {:?}", number1);
            
            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt{ data: number2_bytes};
            println!("number2 : {:?}", number2);

            let mut res = number1 - number2;
            println!("res : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u32::from_be_bytes(res.data);
            assert_eq!(res_u32, case.0 - case.1);

        }
    }

    #[test]
    fn test_64bit_sub_fuzzed(){
        use crate::BigInt;
        use crate::rand;

        for _ in 0..10000{
            let case = (rand::random::<u64>(),rand::random::<u64>());
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt{ data: number1_bytes};
            println!("number1 : {:?}", number1);
            
            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt{ data: number2_bytes};
            println!("number2 : {:?}", number2);

            let mut res = number1 - number2;
            println!("res : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);
            assert_eq!(res_u32, case.0.wrapping_sub(case.1));

        }
    }
    #[test]
    fn test_64bit_cmp_fuzzed(){
        use crate::BigInt;
        use crate::rand;

        for _ in 0..10000{
            let case = (rand::random::<u64>(),rand::random::<u64>());
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt{ data: number1_bytes};
            println!("number1 : {:?}", number1);
            
            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt{ data: number2_bytes};
            println!("number2 : {:?}", number2);

            let res = number1.cmp(&number2);
            assert_eq!(res, case.0.cmp(&case.1));

        }
    }
}