pub mod guess_the_number;
pub mod datatype_and_variables;

fn main() {

    let x = 22;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scop is: {x}");
    }

    println!("The value of x is: {x}");

    guess_the_number::game();
}
