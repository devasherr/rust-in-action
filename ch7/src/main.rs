use std::io::Read;

use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::to_string as to_json;

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
    let bytes_per_line: usize = 16;
    let mut input: &'static [u8] = br#"
    fn main() {
        println!("Hello, World!");
    }
        "#;

    let mut buffer: Vec<u8> = vec![];
    input.read_to_end(&mut buffer)?;

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

fn main() {
    // serialize();
    hex_view().unwrap();
}
