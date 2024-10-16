use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    guess_the_number();
    convert_between_temperatures();
    nth_fibonacci_number();
}

// Guess the number game 
fn guess_the_number(){
   println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut count:u8 = 0;
    while count < 3 {
        // using the normal loop
        // loop {
        // if count == 3 {
        //     println!("The Secret number is: {secret_number}");
        //     break;
        // }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;},
        }

        count += 1;
    }
    
    println!("The Secret number is: {secret_number}");

}

// Task 1
// Temperature conversion from fahrenheit to celcius
fn convert_between_temperatures(){
    println!("To convert from Fahrenheit to Celcius Press 1");
    println!("To convert from Celcius to Fahrenheit Press 2");

    let mut conv = String::new();
    let mut temp = String::new();
    io::stdin().read_line(&mut conv);
    let conv: u8 = match conv.trim().parse() {
        Ok(num)=>num,
        Err(_)=>0,
    };
  
    if conv == 1 {
        println!("Input temp to in fahrenheit to be converted to celcius");
        io::stdin().read_line(&mut temp);
        let temp: f64 = match temp.trim().parse() {
            Ok(num)=>num,
            Err(_)=>0.0,
        };
        let degree_in_celcius: f64 = (temp-32.0) * 5.0/9.0;
        println!("Your degree in Celcius is: {degree_in_celcius} ");
    }else if conv == 2{
        println!("Input temp to in celcius to be converted to fahrenheit");
        io::stdin().read_line(&mut temp);
        let temp: f64 = match temp.trim().parse() {
            Ok(num)=>num,
            Err(_)=>0.0,
        };
        let degree_in_fahrenheit: f64 = (temp*9.0/5.0) + 32.0;
        println!("Your degree in Fahrenheit is: {degree_in_fahrenheit} ");
    } else{
        println!("You have not picked a valid convertion type");
    }

}

// Task 2  
// Printing out the nth number from a fionacci sequence
fn nth_fibonacci_number(){
    println!("input the nth position of the fibonacci number you to know");
    let mut counter:u8 = 1;
    let mut previous_value: u32 = 0;
    let mut current_value: u32 = 1;
    let mut nth_number = String::new();

    io::stdin().read_line(&mut nth_number);
    let nth_number: u8 = match nth_number.trim().parse() {
        Ok(num)=>num,
        Err(_)=>0,
    };
   
    while counter < nth_number {
        if nth_number == 0 {
            println!("Input can't be 0");
            break;
        };
        let fibonacci_value = previous_value + current_value;
        previous_value = current_value;
        current_value = fibonacci_value;
        counter += 1;
    }

    println!("The {nth_number}th fibonacci is {current_value}")
}