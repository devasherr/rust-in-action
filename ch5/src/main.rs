use std::{mem::transmute, num};

fn int_vs_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    // they have the same bit pattern, the type system makes the distinction
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

fn f32_as_u32() {
    let a: f32 = 42.42;

    // unsafe basically tell compiler "i've got this"
    // not advised to use unsafe
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
    assert_eq!(a, b);
}

fn integer_overflow() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            println!("\n");
        }
    }
}

fn endianness() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b)
    // most modern hardware use little endian
}

fn float_number_components(n: f32) {
    // floating number are represented in scientific notation
    // sign bit - exponent - mantissa (the body)

    // isolate sign bit
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;
    println!("sign bit: {}", sign_bit);

    // isolate exponent
    let n_bits = n.to_bits();
    let exponent = n_bits >> 23; // remove mantissa part
    let exponent = exponent & 0xff;
    let exponent = (exponent as i32) - 127;
    println!("exponent: {}", exponent);

    // mantissa - a bit more complex to extract
    // multiply each bit with its weight and sum the result
    // first bit's weight is 0.5, and next is half and so on ...
    let n_bits = n.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("mantissa: {}", mantissa);
}

// fixed point format that uses a singly byte
// basically its a way to represent fractional numbers using integers (approximation)
// actual value = stored integer / 128 (128 because of i8)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Q7(i8); // tuble struct syntax

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::Q7;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7::from(1.0));
        assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}

fn mock_rand(n: u8) -> f32 {
    // division is slow
    n as f32 / 255.0
}

fn mock_rand_2(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15; // align input byte n to 32 bits, then increase value
    let f32_bits = base | large_n; // OR merging base with input
    let m = f32::from_bits(f32_bits);

    2.0 * (m - 0.5) // normalize output range
}

fn generate_random() {
    println!(
        "max of input range: {:08b} -> {:?}",
        0xff,
        mock_rand_2(0xff)
    );
    println!(
        "mid of input range: {:08b} -> {:?}",
        0x7f,
        mock_rand_2(0x7f)
    );
    println!(
        "max of input range: {:08b} -> {:?}",
        0x00,
        mock_rand_2(0x00)
    );
}

fn main() {
    // int_vs_int();
    // f32_as_u32();
    // integer_overflow();
    // endianness();
    // float_number_components(42.42);
    generate_random();
}
