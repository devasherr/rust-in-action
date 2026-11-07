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

fn mem_scan() {
    let mut n_nonzero = 0;
    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}

fn print_addresses() {
    let local_str = "a";
    let local_int = 3;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);

    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
}

fn row_wise_sum(data: &[u32], n: usize) -> u64 {
    let mut res = 0_u64;
    for i in 0..n {
        for j in 0..n {
            res += data[i * n + j] as u64;
        }
    }
    res
}

fn col_wise_sum(data: &[u32], n: usize) -> u64 {
    let mut res = 0_u64;
    for j in 0..n {
        for i in 0..n {
            res += data[i * n + j] as u64;
        }
    }
    res
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

    // mem_scan();
    print_addresses();

    // virtual memory to physical memory
    // CPU contain a memroy managment unit (MMU)
    // memory lookup is cached in Translation lookaside buffer (TLB)
    // Virtual addresse are grouped into blocks called pages (typically 4KB)

    // ## tips for optimiaztion related to virtula memory
    // - keep hot working portions of program with 4KB (faster lookup)
    // - if 4KB is unreasonable, next target should be 4KB * 100
    // - avoid deeply nested data structures with pointer spaghetti
    // - test ordering of your nested loops, column wise or row-wise
    // (col_wise_sum is far worse than row_wise_sum because 2D array in rust is row-major
    // meaning arr[i][j] and arr[i][j+1] sit next to each other in memory)
}
