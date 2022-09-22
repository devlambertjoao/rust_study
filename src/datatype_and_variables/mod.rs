pub fn variables() {
    let x = "My imutable string";
    let mut x_mutable = "My mutable string";

    println!("X Value: {}", x);

    // This will generate error:
    // x = "asdadsd";
    // But reasign not:
    let x = "my new imutable";

    println!("{}. This is my x_mutable: {}", x, x_mutable);
    x_mutable = "New mutable value :)";

    println!("My new mutable value x_mutable: {}", x_mutable);

    // Other way to mutable;
    const THEE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THEE_HOURS_IN_SECONDS);
}

pub fn floating_points() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("My f64: {}, my f32: {}", x, y);
    println!("f64 have double precision");
}

pub fn numeric_operations() {
    // Addiction
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; //Will result in 0;

    // Remainder
    let remainder = 43 % 5;

    println!(
        "Add: {}, Sub: {}, Multi: {}, Div (Quotient): {}, Div (Floored): {}, Remainder: {}",
        sum, difference, product, quotient, floored, remainder
    );
}

pub fn boolean_types() {
    let t = true; // Implicit
    let f: bool = false; // Explicit
    
    println!("My implicit: {}, my explicit boolean: {}", t, f);
}
