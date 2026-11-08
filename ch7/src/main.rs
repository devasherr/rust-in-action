use std::env;
use std::fs::File;
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

fn main() {
    // serialize();
    hex_view().unwrap();
}
