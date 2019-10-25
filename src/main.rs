use std::io;

fn convert_to_celsius(num: f64) {
    let result = (num - 32.0) / 1.8;
    println!("result: {} °C", result);
}

fn convert_to_fahreinheit(num: f64) {
    let result = num * 9.0/5.0 + 32.0;
    println!("result: {} °F", result);
}

fn main() {
    println!("What is the temperature you wanna convert ?");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f64 = number.trim().parse().unwrap();

    println!("Do you want converted in Fahrenheit or Celsius ? (answer by F or C)");

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    if response.trim() == "C" {
        convert_to_celsius(number);
    } else if response.trim() == "F" {
        convert_to_fahreinheit(number);
    } else {
        println!("Wrong answer, answer by F or C");
    }
}
