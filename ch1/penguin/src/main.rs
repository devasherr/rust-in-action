use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little P, 33
    Yellow-eyed p, 65
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }

    let a = 10; // stack
    let b = Box::new(10); // boxed value (heap)
    let c = Rc::new(Box::new(10)); // boxed value wrapped within a reference counter
    let d = Arc::new(Mutex::new(10)); // value wrapped in atomic reference counter and protected by mutual exclusion lock

    println!("{}, {}, {}, {:?}", a, b, c, d);
}
