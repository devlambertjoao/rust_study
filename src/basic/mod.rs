pub mod basic {
    use std::io;

    pub fn init() {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new(); //Mutable
        let apple = 5; //Imutable

        // By default rust variables is imutable (Constants);

        // How to recive user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        println!("You have {apple} Apples!");
        println!("Or... You have {} Apples!", apple);
    }
}
