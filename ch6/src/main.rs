static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn is_strong(x: String) -> bool {
    x.len() > 6
}

fn is_strong_general<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 6
}

fn heap_stuff() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;
    drop(a);

    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2)
}

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

    // rust has a bunch of pointers (pointer-like) types
    // raw pointer, Box<T>, Rc<T>, Arc<T>
    // Cell<T>, RefCell<T>, Cow<T>, String
    // Arc<T>, RawVec<T>, Unique<T>, Shared<T>

    // the two novel attributes that appeare in most of them are interior mutability and shared ownership
    // interior mutability = useful to fake immutability
    // shared ownership = multiple parts of program share access to a single resource
    // both (interior mutability and shared ownership) come with a runtime cost

    let x = String::from("strongpass");
    println!("{}", is_strong(x));

    let x = String::from("strongpass");
    println!("{}", is_strong_general(x));
    println!("{}", is_strong_general("alsostringpass"));

    heap_stuff();
}
