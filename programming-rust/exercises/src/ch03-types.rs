pub fn run() {
    println!("Chapter 3: Basic Types");

    integer_types();
    integer_arithmetic();
    floating_point_types();
    booleans();
    characters();
    tuples();
    arrays();
    vectors();
    slices();
    string_literals();
    strings_in_memory();
    strings();
    type_aliases();
}

fn integer_types() {
    // Integer literals can use type suffixes, base prefixes, and separators.
    let decimal = 116_i8;
    let hexadecimal = 0xcafe_u32;
    let binary = 0b0010_1010;
    let octal = 0o106;
    assert_eq!(decimal, 116);
    assert_eq!(hexadecimal, 51_966);
    assert_eq!(binary, 42);
    assert_eq!(octal, 70);

    // A byte literal is another way to write a u8 value.
    let letter = b'A';
    let newline = b'\n';
    let escape = b'\x1b';
    assert_eq!(letter, 65_u8);
    assert_eq!(newline, 10_u8);
    assert_eq!(escape, 27_u8);

    // Type casting
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    // Out-of-range conversions truncate the value to the destination width.
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}

fn integer_arithmetic() {
    // Checked operations return None instead of overflowing.
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128_i8).checked_div(-1), None);

    // Wrapping operations reduce the result to the type's range.
    assert_eq!(100_u16.wrapping_mul(200), 20_000);
    assert_eq!(500_u16.wrapping_mul(500), 53_392);
    assert_eq!(500_i16.wrapping_mul(500), -12_144);
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // Saturating operations clamp the result to the nearest limit.
    assert_eq!(32760_i16.saturating_add(10), i16::MAX);
    assert_eq!((-32760_i16).saturating_sub(10), i16::MIN);

    // Overflowing operations return the wrapped result and an overflow flag.
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}

fn floating_point_types() {
    let inferred = -1.5625;
    let single_precision = 40_f32;
    let double_precision = 9.109_383_56e-31_f64;

    assert_eq!(inferred, -25.0 / 16.0);
    assert_eq!(single_precision, 40.0);
    assert!(double_precision.is_finite());

    assert!((-1.0 / f32::INFINITY).is_sign_negative());
    let smallest = f32::MIN;
    assert_eq!(-smallest, f32::MAX);
    assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.0);
    assert_eq!((-1.01_f64).floor(), -2.0);
}

fn booleans() {
    let left = 2;
    let right = 5;
    assert!(left < right);
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    let value = 7;
    assert!(value != 0);
}

fn characters() {
    // A char can contain one Unicode character, not just ASCII.
    let rust = '錆';
    // This character's Unicode code point is U+0CA0.
    let look_of_disapproval = 'ಠ';

    // A Unicode character can be written directly as a char literal.
    assert_eq!(rust, '錆');
    // Casting a char to an integer returns its Unicode code point.
    assert_eq!('*' as i32, 42);
    // U+0CA0 fits completely in a u16.
    assert_eq!(look_of_disapproval as u16, 0x0ca0);
    // An i8 keeps only the lowest eight bits.
    assert_eq!(look_of_disapproval as i8, -0x60);

    // An asterisk is not an alphabetic Unicode character.
    assert!(!'*'.is_alphabetic());
    // Unicode-aware methods recognize Greek beta as alphabetic.
    assert!('β'.is_alphabetic());
    // Convert a digit char using radix 10.
    assert_eq!('8'.to_digit(10), Some(8));
    // U+0CA0 needs three bytes when encoded as UTF-8.
    assert_eq!(look_of_disapproval.len_utf8(), 3);
    // Convert a numeric digit to a char.
    assert_eq!(char::from_digit(2, 10), Some('2'));
    // A valid code point becomes Some(char).
    assert_eq!(char::from_u32(0x0ca0), Some(look_of_disapproval));
}

fn tuples() {
    let year_founded = ("Brazil", 1985);
    assert_eq!(year_founded.0, "Brazil");
    assert_eq!(year_founded.1, 1985);

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let singleton = ("lonely hearts",);
    assert_eq!(singleton.0, "lonely hearts");

    let unit = ();
    assert_eq!(unit, ());
}

fn arrays() {
    // An array's length is part of its type: [element type; length].
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // [value; length] creates an array containing repeated values.
    let mut sieve = [true; 10_000];
    for i in 2..100 {
        if sieve[i] {
            let mut multiple = i * i;
            while multiple < sieve.len() {
                sieve[multiple] = false;
                multiple += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // Array references automatically become slices when calling slice methods.
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

fn vectors() {
    // Vec<T> is a growable, heap-allocated sequence.
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30_030);

    let pixel_buffer = new_pixel_buffer(2, 3);
    assert_eq!(pixel_buffer, [0; 6]);

    let mut palindrome = vec!["step", "on", "no", "pets"];
    assert_eq!(palindrome, ["step", "on", "no", "pets"]);

    let collected: Vec<i32> = (0..5).collect();
    assert_eq!(collected, [0, 1, 2, 3, 4]);

    palindrome.reverse();
    assert_eq!(palindrome, ["pets", "no", "on", "step"]);

    let mut reserved = Vec::with_capacity(2);
    assert_eq!(reserved.len(), 0);
    assert_eq!(reserved.capacity(), 2);
    reserved.push(1);
    reserved.push(2);
    reserved.push(3);
    assert_eq!(reserved.len(), 3);
    assert!(reserved.capacity() >= 3);

    let mut values = vec![10, 20, 30, 40, 50];
    values.insert(3, 35);
    assert_eq!(values, [10, 20, 30, 35, 40, 50]);
    values.remove(1);
    assert_eq!(values, [10, 30, 35, 40, 50]);

    let mut gems = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(gems.pop(), Some("Glass Gem"));
    assert_eq!(gems.pop(), Some("Snow Puff"));
    assert_eq!(gems.pop(), None);
}

fn new_pixel_buffer(rows: usize, columns: usize) -> Vec<u8> {
    vec![0; rows * columns]
}

fn slices() {
    let vector = vec![0.0, 0.707, 1.0, 0.707];
    let array = [0.0, -0.707, -1.0, -0.707];

    // A slice borrows a contiguous portion of an array or vector.
    let vector_slice: &[f64] = &vector;
    let array_slice: &[f64] = &array;
    assert_eq!(vector_slice, [0.0, 0.707, 1.0, 0.707]);
    assert_eq!(array_slice, [0.0, -0.707, -1.0, -0.707]);

    // Range syntax selects a smaller slice without copying its elements.
    assert_eq!(&vector[0..2], [0.0, 0.707]);
    assert_eq!(&array[2..], [-1.0, -0.707]);
    assert_eq!(&vector_slice[1..3], [0.707, 1.0]);

    // The same function accepts slices borrowed from either collection type.
    let tolerance = 1e-12;
    assert!((slice_sum(&vector) - 2.414).abs() < tolerance);
    assert!((slice_sum(&array) + 2.414).abs() < tolerance);
}

fn slice_sum(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

fn string_literals() {
    let speech = "\"Ouch!\" said the well.\n";
    assert!(speech.starts_with('"'));
    assert!(speech.ends_with('\n'));

    // A backslash joins source lines without adding a newline or indentation.
    let joined = "It was a bright, cold day in April, and \
                  there were four of us—\
                  more or less.";
    assert_eq!(
        joined,
        "It was a bright, cold day in April, and there were four of us—more or less."
    );

    // Raw strings preserve backslashes instead of treating them as escapes.
    let windows_path = r"C:\Program Files\Gorillas";
    assert_eq!(windows_path, "C:\\Program Files\\Gorillas");

    // Pound signs let a raw string contain double quotes.
    let quoted = r#"He said, "Rust!""#;
    assert_eq!(quoted, "He said, \"Rust!\"");

    // A byte string is an array of u8 values rather than Unicode text.
    let method: &[u8; 3] = b"GET";
    assert_eq!(method[0], b'G');
    assert_eq!(method[1], b'E');
    assert_eq!(method[2], b'T');
}

fn strings_in_memory() {
    // String owns a growable UTF-8 buffer; &str borrows UTF-8 text.
    let noodles = "noodles".to_string();
    let oodles: &str = &noodles[1..];
    let poodles: &str = "ಠ_ಠ";
    assert_eq!(oodles, "oodles");

    // String lengths count UTF-8 bytes, not Unicode characters.
    assert_eq!(poodles.len(), 7);
    assert_eq!(poodles.chars().count(), 3);
}

fn strings() {
    // Several common operations create owned String values.
    let error_message = "too many pets".to_string();
    let owned_message = "too many pets".to_owned();
    assert_eq!(error_message, owned_message);

    let latitude = format!("{}°{:02}′{:02}″N", 24, 5, 23);
    assert_eq!(latitude, "24°05′23″N");

    let words = ["veni", "vidi", "vici"];
    assert_eq!(words.concat(), "venividivici");
    assert_eq!(words.join(", "), "veni, vidi, vici");

    let mut greeting = String::from("hello");
    greeting.push(',');
    greeting.push_str(" world");
    assert_eq!(greeting, "hello, world");

    // Most text-processing methods are defined on str and work on String too.
    assert_eq!("ONE".to_lowercase(), "one");
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace('ಠ', "■"), "■_■");
    assert_eq!("    clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with('v'));
    }

    // Visually equivalent Unicode text can have different code point sequences.
    let composed = "th\u{e9}";
    let decomposed = "the\u{301}";
    assert_ne!(composed, decomposed);
}

type Bytes = Vec<u8>;

fn type_aliases() {
    // A type alias is another name for an existing type, not a new type.
    let encoded: Bytes = vec![82, 117, 115, 116];
    assert_eq!(encoded, b"Rust");
}
