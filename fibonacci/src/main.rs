use std::io;

fn main() {
    loop {
        println!("Input number!");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        print_fibonacci_number(number);
        break;
    }
}

fn print_fibonacci_number(number: i32) {
    if number == 0 {
        println!("0")
    } else if number == 1 {
        println!("1")
    } else {
        let mut counter = 2;
        let mut current = 1;
        let mut prev = 0;
        while counter <= number {
            let seq_member = current + prev;
            prev = current;
            current = seq_member;
            counter = counter + 1;
            if counter == number + 1 {
                println!("{seq_member}")
            }
        }
    }
}
