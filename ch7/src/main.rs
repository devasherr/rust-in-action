use core::num;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::Read;

use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::{json, to_string as to_json};

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn serialize() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = to_json(&calabar);
    let as_cbor = to_cbor(&calabar);

    println!("{:?}", as_json);
    println!("{:?}", as_cbor);
}

fn hex_view() -> std::io::Result<()> {
    let fname_arg = env::var("FILE_NAME");
    let fname = fname_arg.expect("usage: fview FILENAME");

    let mut f = File::open(fname).expect("unable to open file");

    let bytes_per_line: usize = 16;

    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(bytes_per_line) {
        print!("0x{:08x}", position_in_input);
        for byte in line {
            print!("{:02x} ", byte)
        }

        position_in_input += bytes_per_line;
        println!()
    }

    Ok(())
}

fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones = 0;
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
    }

    (n_ones % 2 == 0) as u8
}

fn hashmap_example() {
    let mut capitals: HashMap<&str, &str> = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Niue", "Alofi");

    println!("{}", capitals["Niue"])
}

fn hashmap_from_json() {
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku'alofa",
        "Tuvalu": "Funafuti"
    });

    println!("{}", capitals["Niue"]);
    println!()
}

fn btreemap_example() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middleburg");
    voc.insert(540_000, "Enkhuizen");

    for (guilder, kamer) in &voc {
        println!("{} invested {}", kamer, guilder)
    }
}

fn main() {
    // serialize();
    // hex_view().unwrap();

    // let abc = b"abc";
    // println!("input: {:?}", abc);
    // println!("output: {:08x}\n", parity_bit(abc));
    //
    // let abc = b"abcd";
    // println!("input: {:?}", abc);
    // println!("output: {:08x}", parity_bit(abc));

    hashmap_example();
    hashmap_from_json();
    btreemap_example();
}
