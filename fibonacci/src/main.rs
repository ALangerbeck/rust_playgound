use std::io;
fn main() {
    loop {
        println!("Input a possitive number");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 0 {
            println!("The fibbonacci number is 0");
            continue;
        } else if input == 1 {
            println!("The fibbonacci number is 1");
            continue;
        }
        let mut num1: u32 = 0;
        let mut num2: u32 = 1;
        for i in 2..input + 1 {
            let fib_num: u32 = num1 + num2;
            num1 = num2;
            num2 = fib_num;
        }
        println!("The fibbonacci number is {num2}")
    }
}
