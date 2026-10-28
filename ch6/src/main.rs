static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);

    // definition of raw pointers: *const T and *mut T
    // raw pointer to a string -> *const String or *mut String
    // raw pointers is strongly discouraged for everyday use

    let a: i64 = 42;
    let a_ptr = &a as *const i64; // handle with care
    println!("a: {} ({:p})", a, a_ptr);

    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr);

    unsafe {
        let new_addr = a_ptr.offset(4);
        println!("{:p} -> {:p}", a_ptr, new_addr)
    }
}
