#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data;
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    Ok(f)
}

fn file_stuff() {
    let mut f1 = File::new_with_data("f1.txt", vec![1, 2, 3, 4]);

    f1 = open(f1).unwrap();
    f1 = close(f1).unwrap();

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} if {} bytes long", f1_name, f1_length);
}

fn file_stuff2() {}

fn main() {
    file_stuff();
}
