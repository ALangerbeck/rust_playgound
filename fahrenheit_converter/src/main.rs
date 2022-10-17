use std::io;

fn main() {
    loop {
        println!("Input a fahrenheit value");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let mut temp: f32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        temp = ((temp - 32.0) * 5.0) / 9.0;
        println!("The converted temperature is {temp}")
    }
}
