use std::f32::EPSILON;
use std::future::poll_fn;
use std::ops::Add;

fn numeric_literals() {
    let one_million: i64 = 1_000_000;
    println!("{}", one_million);

    let forty_twos = [42.0, 42f32, 42_f32];

    println!("{:02}", forty_twos[0]);
}

fn base() {
    // base 2(binary), 8(ocatal), 16(hexa) integers
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {}, {}, {}", three, thirty, three_hundred);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);
}

fn float() {
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
    // assert!(xyz.0 + xyz.1 == xyz.2); // only fails on this one

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;

    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
}

fn flow_control() {
    let collections = [1, 2, 3, 4, 5];
    for i in 0..collections.len() {
        let item = collections[i];
        println!("{item}")

        // this loop approach is not advised for two reasons
        // performance - index bound has to be checked
        // safety - there might be a possibility where collection might change
        // just you a for loop in a case like this mate
    }

    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 100 {
                    break 'outer;
                }
            }
        }
    }
}

fn _match() {
    let niddle = 42;
    let haystack = [1, 2, 5, 42, 14, 132, 1430];
    for item in &haystack {
        let result = match item {
            42 | 132 => "hit",
            _ => "miss",
        };

        if result == "hit" {
            println!("{} {}", item, result)
        }
    }
    println!();
}

fn generics() {
    fn add<T: Add<Output = T>>(i: T, j: T) -> T {
        i + j
    }

    println!("{}", add(4, 8));
    println!("{}", add(4.2, 8.5));
    println!();
}

fn grep_lite() {
    let query = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(query) {
            println!("{}: {}", i + 1, line);
        }
    }
}

fn grep_lite2() {
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let ctx_lines = 2;
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn arrays() {
    let arr = [1; 10]; // ten ones
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];
    for arr in &arrays {
        print!("{:?}", arr);
        for n in arr.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let sum: u8 = arr.iter().sum();

        println!("\t{:?} = {}", arr, sum);
    }
}

fn main() {
    // numeric_literals();
    // base();
    // float();
    // flow_control();
    // _match();
    // generics();
    // arrays();
    // grep_lite();
    grep_lite2();
}
