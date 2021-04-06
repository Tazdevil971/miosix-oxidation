#![crate_type = "staticlib"]

#[no_mangle]
pub extern "C" fn rust_main() {
    // Show off what is currently working!

    use std::time::Instant;
    use std::io::Read;
    use std::fs;

    // Allocations
    let vec = vec![
        10, 20, 30, 40, 50
    ];

    // Stdio
    println!("Hi from rust!");
    println!("{:#?}", vec);

    // Time
    println!("{:?}", Instant::now());
    println!("{:?}", Instant::now());

    // Fs
    for dir in fs::read_dir("/dev").unwrap() {
        println!("{:?}", dir);
    }

    let mut zero = fs::File::open("/dev/zero").unwrap();

    let mut buf = [10u8; 8];
    zero.read_exact(&mut buf).unwrap();
    println!("{:?}", buf);
}