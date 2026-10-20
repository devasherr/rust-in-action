use std::f32::EPSILON;

fn main() {
    let a = 30_i32; // let a: i32 = 30;
    println!("{}", a);

    // numeric literals
    let one_million: i64 = 1_000_000;
    println!("{}", one_million);

    let forty_twos = [42.0, 42f32, 42_f32];

    println!("{:02}", forty_twos[0]);

    // base 2(binary), 8(ocatal), 16(hexa) integers
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {}, {}, {}", three, thirty, three_hundred);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);

    // floating point hazards
    // assert!(0.1 + 0.2 == 0.3);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3: {:x}", (abc.2).to_bits());
    println!();
    println!();

    println!("xyz (f64)");
    println!("0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); // only fails on this one

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;

    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON)
}
