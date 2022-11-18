use std::io;

fn main() {
    loop {
        println!("Input temperature");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Input scale (c/f), to convert from");

        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale: char = match scale.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        if scale == 'c' {
            cels_to_far(temperature);
            break;
        } else if scale == 'f' {
            far_to_cels(temperature);
            break;
        } else {
            println!("Scale must be c for Farenheit or c for Celcius");
            break;
        }
    }
}

fn cels_to_far(temperature: i32) {
    let temperature: f64 = temperature as f64 * 1.8 + 32.0;
    println!("{temperature:.1} deegres Farenheit")
}

fn far_to_cels(temperature: i32) {
    let temperature: f64 = (temperature as f64 - 32.0) * 0.5556;
    println!("{temperature:.1} deegres Celcius")
}
