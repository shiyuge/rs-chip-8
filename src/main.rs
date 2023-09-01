mod interp;

fn main() {
    let mut a: [u8; 40] = [0; 40];
    a[0] = 1;
    a[2] = 4;
    let a = a;

    println!("Hello, world! {:?}", a);
}
