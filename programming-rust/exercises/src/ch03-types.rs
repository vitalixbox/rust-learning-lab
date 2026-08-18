pub fn run() {
    println!("Chapter 3: Basic Types");

    numbers();
    tuples();
    arrays_and_slices();
}

fn numbers() {
    let signed: i32 = -42;
    let unsigned: u32 = 42;
    let inferred = 3.5;

    println!("signed: {signed}, unsigned: {unsigned}, float: {inferred}");
}

fn tuples() {
    let book = ("Programming Rust", 3);
    let (title, chapter) = book;

    println!("{title}, chapter {chapter}");
}

fn arrays_and_slices() {
    let primes = [2, 3, 5, 7, 11];
    let first_three: &[i32] = &primes[..3];

    println!("array: {primes:?}");
    println!("slice: {first_three:?}");
}
