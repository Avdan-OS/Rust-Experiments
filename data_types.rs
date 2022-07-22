fn main() {
    println!("Hello, Rust Data types!");

    // Note: I put underscores before `x` because the Rust compiler gave a bunch of "unused variable" warnings.

    //////////////////////// PRIMITIVE TYPES ////////////////////////

    // i32 (signed 32-bit integer) (default integer type)
    let _x: i32 = 2;
    let _x = 2;

    // i8 (signed 8-bit integer)
    let _x: i8 = 2;

    // i16 (signed 16-bit integer)
    let _x: i16 = 2;

    // i64 (signed 64-bit integer)
    let _x: i64 = 2;

    // i128 (signed 128-bit integer)
    let _x: i128 = 2;

    ///////////////////////////////////////////////////////////////////

    // uint (same as signed int, but can't be negative)

    // i32 (signed 32-bit integer)
    let _x: u32 = 2;

    // i8 (signed 8-bit integer)
    let _x: u8 = 2;

    // i16 (signed 16-bit integer)
    let _x: u16 = 2;

    // i64 (signed 64-bit integer)
    let _x: u64 = 2;

    // i128 (signed 128-bit integer)
    let _x: u128 = 2;

    ///////////////////////////////////////////////////////////////////

    // Floating point value (f32 (single precision, 32-bits), f64 (double precision, 64-bits, default))

    let _x: f32 = 10.9;
    let _x: f64 = 10.9;

    ///////////////////////////////////////////////////////////////////

    // Booleans (True = 1, False = 0)

    let _x: bool = true;
    let _x: bool = false;
    let _x = 1;
    let _x = 0;

    ///////////////////////////////////////////////////////////////////

    // Characters (type char, stores one character, uses single quotes)

    let _x: char = 'd';

    //////////////////////// COMPOUND TYPES ////////////////////////

    // Tuples (immutable by default)

    let x: (i32, bool, char) = (1, true, 's');
    println!("{}", x.0);

    // Arrays (immutable by default) (type doesn't have to be defined explicitly)
    let mut x: [i32; 5] = [1, 2, 3, 4, 5];
    x[2] = x[2] * 2;
    println!("{}", x[2]);
}
