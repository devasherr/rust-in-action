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

fn main() {
    // int_vs_int();
    // f32_as_u32();
    // integer_overflow();
    // endianness();
    float_number_components(42.42);
}
