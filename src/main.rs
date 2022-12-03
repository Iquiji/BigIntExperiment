use const_fn_assert::{cfn_assert, cfn_assert_eq};
use rand;
use std::cmp::min;
use std::fmt::Debug;
use std::fmt::Write;
use std::ops::{AddAssign, Div, Mul, Rem, Shl, Sub};
use std::{cmp::Ordering, ops::SubAssign};
use std::{fmt::Display, ops::Add};

fn main() {
    println!("Hello, world!");

    // RSA
    const SIZE: usize = (4096) / 8;
    let rsa_n_str = "bad47a84c1782e4dbdd913f2a261fc8b65838412c6e45a2068ed6d7f16e9cdf4462b39119563cafb74b9cbf25cfd544bdae23bff0ebe7f6441042b7e109b9a8afaa056821ef8efaab219d21d6763484785622d918d395a2a31f2ece8385a8131e5ff143314a82e21afd713bae817cc0ee3514d4839007ccb55d68409c97a18ab62fa6f9f89b3f94a2777c47d6136775a56a9a0127f682470bef831fbec4bcd7b5095a7823fd70745d37d1bf72b63c4b1b4a3d0581e74bf9ade93cc46148617553931a79d92e9e488ef47223ee6f6c061884b13c9065b591139de13c1ea2927491ed00fb793cd68f463f5f64baa53916b46c818ab99706557a1c2d50d232577d1";
    let rsa_n = BigInt::<SIZE>::from_hex_string(rsa_n_str);
    println!("rsa_n: {:?}", rsa_n);
    let outstring = rsa_n.to_hex_string();
    // assert_eq!(rsa_n_str, outstring.to_lowercase());

    let rsa_d_str = "40d60f24b61d76783d3bb1dc00b55f96a2a686f59b3750fdb15c40251c370c65cada222673811bc6b305ed7c90ffcb3abdddc8336612ff13b42a75cb7c88fb936291b523d80acce5a0842c724ed85a1393faf3d470bda8083fa84dc5f31499844f0c7c1e93fb1f734a5a29fb31a35c8a0822455f1c850a49e8629714ec6a2657efe75ec1ca6e62f9a3756c9b20b4855bdc9a3ab58c43d8af85b837a7fd15aa1149c119cfe960c05a9d4cea69c9fb6a897145674882bf57241d77c054dc4c94e8349d376296137eb421686159cb878d15d171eda8692834afc871988f203fc822c5dcee7f6c48df663ea3dc755e7dc06aebd41d05f1ca2891e2679783244d068f";
    let rsa_d = BigInt::<SIZE>::from_hex_string(rsa_d_str);
    println!("rsa_d: {:?}", rsa_d);
    let outstring = rsa_d.to_hex_string();
    // assert_eq!(rsa_d_str, outstring.to_lowercase());

    let rsa_e_str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010001";
    let rsa_e = BigInt::<SIZE>::from_hex_string(rsa_e_str);
    println!("rsa_e: {:?}", rsa_e);
    let outstring = rsa_e.to_hex_string();
    // assert_eq!(rsa_e_str, outstring.to_lowercase());

    let message = BigInt::<SIZE>::from_hex_string("0C");
    println!("message = {:?}", message);
    let encrypted = message.clone().mod_pow(&rsa_e, &rsa_n);
    println!("mod_pow (c) = {:?}", encrypted);

    let decrypted = encrypted.mod_pow(&rsa_d, &rsa_n);
    println!("decrypted = {:?}", decrypted);

    assert_eq!(message, decrypted);

    // let number = BigInt::<4>::from_hex_string("01FEFE01");
    // assert_eq!(number.to_hex_string(),"01FEFE01");
    // let number1 = BigInt::<4>::from_hex_string("00_00_FF_FF");
    // let number2 = BigInt::<4>::from_hex_string("00_00_00_FF");
    // println!("Mul 1: {:?}", number1);
    // println!("Mul 2: {:?}", number2);
    // println!("Debug: {:?}", number2.shl(15));
    // let mut accum = BigInt::<4>::new();
    // for i in 0..16{
    //     accum = accum + (number2.shl(i));
    // }
    // println!("Expected: {:?}", number);

    // println!("Manual: {:?}", accum);

    // println!("Debug: {:?}", number1 / number2);
    // println!("Debug: {:?}", number2);
    // println!("Debug: {:?}", number2.shl(0));

    // println!("Debug: {:?}", number.shl(1));
    // println!("Debug: {:?}", number2);
    // println!("Debug: {:?}", number2.shl(1));
    // println!("Debug: {:?}", number2.shl(2));
    // println!("Debug: {:?}", number2.shl(3));
    // println!("Debug: {:?}", number2.shl(4));
    // println!("Debug: {:?}", number2.shl(5));
    // println!("Debug: {:?}", number2.shl(6));
    // println!("Debug: {:?}", number2.shl(7));
    // println!("Debug: {:?}", number2.shl(8).shr(8));
    // println!("Debug: {:?}", number2.shl(9).shr(9));
    // println!("Debug: {:?}", number.shl(16).shr(16));
    // let double = number + number2;
    // println!("Debug: {:?}", double);
    // let number3 = number - number2;
    // println!("Debug: {:?}", number3);
}

// struct RSA<const SIZE: usize> {
//     p: BigInt<SIZE>,
//     q: BigInt<SIZE>,
//     n: BigInt<SIZE>,
//     e: BigInt<SIZE>,
//     d: BigInt<SIZE>,
// }
// impl<const SIZE: usize> RSA<{ SIZE }> {
//     fn generate() -> Self {
//         unimplemented!()
//     }
//     fn from_n_d_e() -> Self {
//         unimplemented!()
//     }
//     fn get_public_key(&self) -> (BigInt<SIZE>, BigInt<SIZE>) {
//         (self.e, self.n)
//     }
//     fn get_private_key(&self) -> (BigInt<SIZE>, BigInt<SIZE>) {
//         (self.d, self.n)
//     }
//     fn encrypt(&self, data: BigInt<SIZE>) -> BigInt<SIZE> {
//         unimplemented!()
//     }
//     fn decrypt(&self, data: BigInt<SIZE>) -> BigInt<SIZE> {
//         unimplemented!()
//     }
// }

// const ELEMENT_BIT_SIZE: usize = 8;

#[derive(Clone, PartialEq, Eq)]
pub struct BigInt<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> BigInt<{ SIZE }> {
    pub const ZERO: Self = BigInt::new();
    pub const ONE: Self = BigInt::from_u8(1);
    pub const TWO: Self = BigInt::from_u8(2);
    pub const TWO_FIVE_SIX: Self = BigInt::two_five_six();

    pub const fn new() -> Self {
        BigInt { data: [0; SIZE] }
    }

    pub const fn from_u8(num: u8) -> Self {
        let mut int = Self::new();
        int.data[0] = num;
        int
    }

    pub fn from_hex_string(string: &str) -> Self {
        // println!("parse...");
        let mut data = [0; SIZE];
        let str_filtered = string
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>();
        // println!("string filtered: '{}'",str_filtered);
        let str_iter = str_filtered.chars();
        for idx in (0..SIZE).rev() {
            let hex_4 = str_iter
                .clone()
                .chain(vec!['0'].iter().cloned().cycle())
                .skip((SIZE - idx) * 2 - 2)
                .take(2)
                .collect::<String>();
            // println!("hex_4: {hex_4}");
            let parsed =
                u8::from_str_radix(&hex_4, 16).expect("Error Parsing Hex String into BigInt");

            data[SIZE - idx - 1] = parsed;
        }
        BigInt { data }
    }

    fn split(&self, at: usize) -> (BigInt<SIZE>, BigInt<SIZE>) {
        // println!("SIZE, {}",SIZE);
        // let half = SIZE / 2;
        // println!("HALF, {}",half);
        // cfn_assert_eq!(HALF, SIZE / 2);
        let mut high = BigInt::<SIZE> { data: [0; SIZE] };
        let mut low = BigInt::<SIZE> { data: [0; SIZE] };
        for idx in 0..SIZE {
            if idx < at {
                low.data[idx] = self.data[idx];
            } else {
                high.data[idx - at] = self.data[idx];
            }
        }
        // println!("before: {:?}",self);
        // println!("high: {:?}",high);
        // println!("low: {:?}",low);
        (high, low)
    }

    const fn two_five_six() -> BigInt<SIZE> {
        cfn_assert!(SIZE > 1);
        let mut result = BigInt::ZERO;
        result.data[1] = 1;
        result
    }

    fn size(&self) -> usize{
        for idx in (0..SIZE).rev(){
            if self.data[idx] != 0{
                return idx + 1;
            }
        }
        1
    }
}

// uncomment to break the build :D
// const _: BigInt<1> = BigInt::<1>::two_five_six();

impl<const SIZE: usize> AddAssign<&Self> for BigInt<{ SIZE }> {
    fn add_assign(&mut self, rhs: &Self) {
        let mut carry_flag = false;

        for idx in 0..SIZE {
            // println!("carry flag: {}", carry_flag);
            let step = self.data[idx].checked_add(rhs.data[idx]);
            if let Some(step) = step {
                self.data[idx] = if let Some(step_inner) = step.checked_add(u8::from(carry_flag)) {
                    carry_flag = false;
                    step_inner
                } else {
                    carry_flag = true;
                    step.wrapping_add(1)
                };
            } else {
                // println!("overflow in add");
                self.data[idx] = self.data[idx].wrapping_add(rhs.data[idx]) + u8::from(carry_flag);

                carry_flag = true;
            }
        }

        // eprintln!("carry at end of add: {}", carry_flag);
        if carry_flag {
            println!("overflow in add!");
        }
    }
}

impl<const SIZE: usize> Add for &BigInt<{ SIZE }> {
    type Output = BigInt<{ SIZE }>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<const SIZE: usize> SubAssign<&Self> for BigInt<{ SIZE }> {
    fn sub_assign(&mut self, rhs: &Self) {
        let mut carry_flag = false;

        for idx in 0..SIZE {
            // println!("carry flag: {}", carry_flag);
            let step = self.data[idx].checked_sub(rhs.data[idx]);
            // println!("{} - {} = {:?}", self.data[idx], rhs.data[idx], step);
            if let Some(step) = step {
                self.data[idx] = if let Some(step_inner) = step.checked_sub(u8::from(carry_flag)) {
                    carry_flag = false;
                    step_inner
                } else {
                    carry_flag = true;
                    step.wrapping_sub(1)
                };
            } else {
                // println!("underflow in sub");
                self.data[idx] = self.data[idx].wrapping_sub(rhs.data[idx]) - u8::from(carry_flag);

                carry_flag = true;
            }
        }

        // eprintln!("carry at end of sub: {}", carry_flag);
    }
}

impl<const SIZE: usize> Sub for &BigInt<{ SIZE }> {
    type Output = BigInt<{ SIZE }>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<const SIZE: usize> Mul for &BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn mul(self, rhs: Self) -> Self::Output {
        if cfg!(feature = "karatsuba") {
            karatsuba(self, rhs, SIZE)
        } else {
            schoolbook(self, rhs)
        }
    }
}

pub fn schoolbook<const SIZE: usize>(a: &BigInt<SIZE>, b: &BigInt<SIZE>) -> BigInt<SIZE> {
    let mut output = BigInt::ZERO;
    for idx in 0..SIZE {
        for bit_idx in 0..8 {
            // println!("bit: {}", idx * 8 + bit_idx);
            // if bit is set
            if (a.data[idx] & (1 << bit_idx)) != 0 {
                let shift_amount = idx * 8 + bit_idx;
                // println!("shift_amount : {}",shift_amount);
                output += &b.shl(shift_amount);
            }
        }
    }
    // println!("sum_elements: {:#?}", sum_elements);

    // let mut output = BigInt::<SIZE>::new();
    // for element in sum_elements {
    //     output = output + element;
    //     // println!("added : {:?}", element);
    //     // println!("output: {:?}", output);
    // }
    // println!("mul");
    output
}

pub fn karatsuba<const SIZE: usize>(a: &BigInt<SIZE>, b: &BigInt<SIZE>, split_at: usize) -> BigInt<SIZE> {
    if a <= &BigInt::from(255) || b <= &BigInt::from(255) {
        return schoolbook(a, b);
    }
    
    let size = min(a.size(), b.size());
    // println!("self {:?}, counted size: {} ",a,a.size());
    let split_point = size / 2;
    // println!("\nsplit_point: {}", split_point);
    let (high1, low1) = a.split(split_point);
    let (high2, low2) = b.split(split_point);

    let z0 = karatsuba(&low1, &low2, split_point);
    let z1 = karatsuba(&(&low1 + &high1), &(&low2 + &high2), split_point);
    let z2 = karatsuba(&high1, &high2, split_point);


    let res_1 = z2.shl(2 * 8 * split_point);
    let res_2 = (&(&z1 - &z2) - &z0).shl(split_point * 8);

    &(&res_1 + &res_2) + &z0
}

impl<const SIZE: usize> Div for BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == BigInt::ZERO {
            panic!("Divide by Zero!");
        }
        let mut quotient = BigInt::new();
        let mut remainder = BigInt::new();

        for idx in (0..SIZE).rev() {
            for bit_idx in (0..8).rev() {
                remainder = remainder.shl_once();
                remainder.data[0] |= (self.data[idx] & (1 << bit_idx)) >> bit_idx;
                // let bit_at = idx * 8 + bit_idx;
                // println!("bit: {}", idx * 8 + bit_idx);
                // if bit is set
                if remainder >= rhs {
                    remainder -= &rhs;
                    quotient.data[idx] |= 1 << bit_idx;
                    // println!("shift_amount : {}",shift_amount);
                    // sum_elements.push(rhs.shl(shift_amount));
                }
            }
        }
        quotient
    }
}

impl<const SIZE: usize> Rem for &BigInt<{ SIZE }> {
    type Output = BigInt<SIZE>;

    fn rem(self, rhs: Self) -> Self::Output {
        if *rhs == BigInt::ZERO {
            panic!("Modulo by Zero!");
        }
        let mut quotient: BigInt<SIZE> = BigInt::new();
        let mut remainder: BigInt<SIZE> = BigInt::new();

        for idx in (0..SIZE).rev() {
            for bit_idx in (0..8).rev() {
                remainder = remainder.shl_once();
                remainder.data[0] |= (self.data[idx] & (1 << bit_idx)) >> bit_idx;
                // let bit_at = idx * 8 + bit_idx;
                // println!("bit: {}", idx * 8 + bit_idx);
                // if bit is set
                if remainder >= *rhs {
                    remainder -= rhs;
                    quotient.data[idx] |= 1 << bit_idx;
                    // println!("shift_amount : {}",shift_amount);
                    // sum_elements.push(rhs.shl(shift_amount));
                }
            }
        }
        remainder
    }
}

impl<const SIZE: usize> Ord for BigInt<{ SIZE }> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for idx in (0..SIZE).rev() {
            if self.data[idx] > other.data[idx] {
                return std::cmp::Ordering::Greater;
            }
            if self.data[idx] < other.data[idx] {
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
    pub fn shl_once(self) -> Self {
        let mut output = [0; SIZE];
        let mut carry = false;
        for idx in 0..SIZE {
            output[idx] = (self.data[idx] << 1) + u8::from(carry);
            // overflow
            if (self.data[idx] << 1) < self.data[idx] {
                carry = true;
            } else {
                carry = false;
            }
        }

        BigInt { data: output }
    }

    pub fn shl(&self, amount: usize) -> Self {
        let mut output = self.clone();
        for _ in 0..amount {
            output = output.shl_once();
        }
        output
    }

    pub fn shr_once(self) -> Self {
        let mut output = [0; SIZE];
        let mut carry = false;
        for idx in (0..SIZE).rev() {
            output[idx] = (self.data[idx] >> 1) + if carry { 0b1000_0000 } else { 0 };
            // overflow
            if (self.data[idx] & 1) == 1 {
                carry = true;
            } else {
                carry = false;
            }
        }

        BigInt { data: output }
    }

    pub fn shr(self, amount: usize) -> Self {
        let mut output = self;
        for _ in 0..amount {
            output = output.shr_once();
        }
        output
    }
}

impl<const SIZE: usize> From<u8> for BigInt<{ SIZE }> {
    fn from(num: u8) -> Self {
        Self::from_u8(num)
    }
}
/*
https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

*/
impl<const SIZE: usize> BigInt<{ SIZE }> {
    fn mod_pow(self, exp: &Self, modulus: &Self) -> Self {
        let mut exp = exp.clone();
        if BigInt::ONE == *modulus {
            return BigInt::ZERO;
        }
        let mut result: BigInt<SIZE> = BigInt::ONE;
        let mut base = &self % modulus;
        let mut timer = std::time::Instant::now();
        let mut iter = 0;
        while exp > BigInt::ZERO {
            println!("Took: {:?} for {}", timer.elapsed(), iter);
            timer = std::time::Instant::now();
            iter += 1;
            // println!("exp: {:?}",exp);
            // println!("res: {:?}",result);
            if (exp.data[0] & 1) == 1 {
                result = &(&result * &base) % modulus;
            }
            base = &(&base * &base) % modulus;
            exp = exp.shr_once();
        }
        result
    }
}

impl<const SIZE: usize> BigInt<{ SIZE }> {
    fn to_hex_string(&self) -> String {
        let mut res = String::new();
        for idx in (0..SIZE).rev() {
            write!(res, "{:02X?}", self.data[idx]).unwrap();
            // println!("{}",res);
        }
        res
    }
}

impl<const SIZE: usize> Display for BigInt<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex_string())
    }
}

impl<const SIZE: usize> Debug for BigInt<{ SIZE }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = self.data;
        data.reverse();
        write!(f, "{:?}", data)
    }
}

impl From<u32> for BigInt<4> {
    fn from(number: u32) -> Self {
        let mut bytes = number.to_be_bytes();
        bytes.reverse();
        BigInt { data: bytes }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Shl;

    #[test]
    fn test_32bit_sub() {
        use crate::BigInt;

        let cases = [(1u32, 1u32), (0, 0), (u32::MAX, 1), (10000, 1234)];
        for (a, b) in cases {
            let number1 = BigInt::from(a);
            println!("number1 : {:?}", number1);

            let mut number2_bytes = b.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let mut res = &number1 - &number2;
            println!("res : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u32::from_be_bytes(res.data);
            assert_eq!(res_u32, a - b);
        }
    }

    #[test]
    fn test_64bit_shl_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..10000 {
            let case = (rand::random::<u64>(), rand::random::<u8>() as usize / 4);
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut res = number1.shl(case.1);
            println!("res : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);
            assert_eq!(res_u32, case.0.shl(case.1));
        }
    }

    #[test]
    fn test_64bit_sub_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..10000 {
            let case = (rand::random::<u64>(), rand::random::<u64>());
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let mut res = &number1 - &number2;
            println!("res : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);
            assert_eq!(res_u32, case.0.wrapping_sub(case.1));
        }
    }

    #[test]
    fn test_64bit_cmp_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..10000 {
            let case = (rand::random::<u64>(), rand::random::<u64>());
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let res = number1.cmp(&number2);
            assert_eq!(res, case.0.cmp(&case.1));
        }
    }

    #[test]
    fn test_64bit_mul_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..4000 {
            let case = (rand::random::<u32>() as u64, rand::random::<u32>() as u64);
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let mut res = &number1 * &number2;
            println!("res       : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);

            let mut should_be = (case.0 * case.1).to_be_bytes();
            should_be.reverse();
            let should_be = BigInt { data: should_be };
            println!("should_be : {:?}", should_be);

            res.data.reverse();
            let diff = &should_be - &(res);
            println!("diff : {:?}", diff);
            for shift_amount in 0..64 {
                if number2.shl(shift_amount) == diff {
                    println!("Eureka!");
                }
            }


            assert_eq!(res_u32, case.0 * case.1);
        }
    }
    #[test]
    fn test_64bit_div_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..1000 {
            let case = (rand::random::<u64>(), rand::random::<u64>() / 3);
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let mut res = number1 / number2;
            println!("res       : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);

            let mut should_be = (case.0 / case.1).to_be_bytes();
            should_be.reverse();
            let should_be = BigInt { data: should_be };
            println!("should_be : {:?}", should_be);

            assert_eq!(res_u32, case.0 / case.1);
        }
    }

    #[test]
    fn test_64bit_mod_fuzzed() {
        use crate::rand;
        use crate::BigInt;

        for _ in 0..1000 {
            let case = (rand::random::<u64>(), rand::random::<u64>() / 3);
            let mut number1_bytes = case.0.to_be_bytes();
            number1_bytes.reverse();
            let number1 = BigInt {
                data: number1_bytes,
            };
            println!("number1 : {:?}", number1);

            let mut number2_bytes = case.1.to_be_bytes();
            number2_bytes.reverse();
            let number2 = BigInt {
                data: number2_bytes,
            };
            println!("number2 : {:?}", number2);

            let mut res = &number1 % &number2;
            println!("res       : {:?}", res);
            // this is because u32 spit want bytes weird
            res.data.reverse();
            let res_u32 = u64::from_be_bytes(res.data);

            let mut should_be = (case.0 % case.1).to_be_bytes();
            should_be.reverse();
            let should_be = BigInt { data: should_be };
            println!("should_be : {:?}", should_be);

            // let diff = should_be - number1 * number2;
            // println!("diff : {:?}", diff);
            // for shift_amount in 0..64 {
            //     if number2.shl(shift_amount) == diff {
            //         println!("Eureka!");
            //     }
            // }

            assert_eq!(res_u32, case.0 % case.1);
        }
    }
}
