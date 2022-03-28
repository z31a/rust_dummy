
use std::io;

fn main() {
    println!("Let's convert the temperatures");

    println!(" Enter your choice : \n 1.Fahrenheit to Celsius \n 2.Celsius to Fahrenheit \n >>> ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading the choice value");

    let choice: i32 = choice.trim().parse().expect("Error parsing choice value");

    if choice == 1 {
        read_fah();
    } else {
        read_cel();
    }
}

fn read_fah() {
    println!("Enter temperature value in fahrenheit : ");

    let flag = true;

    let mut fah = String::new();

    io::stdin()
        .read_line(&mut fah)
        .expect("Error getting Fahrehheit value");

    let fah: f64 = fah.trim().parse().expect("Error parsing fahrenheit value");

    println!("The given {} temperature in fahrenehit is {} in celsius.", fah, temp_conv(fah, flag));
}

fn read_cel() {
    println!("Enter temperature value in celsius : ");

    let flag = false;

    let mut cel = String::new();

    io::stdin()
        .read_line(&mut cel)
        .expect("Error getting celsius value");

    let cel: f64 = cel.trim().parse().expect("Error parsing celsius value");

    println!("The given {} temperature in celsius is {} in fahrenheit.", cel, temp_conv(cel, flag));
}

fn temp_conv(temp_value: f64, flag: bool) -> f64 {

    let frac: f64 = 5.00 / 9.00;

    if flag {
        //fahrenheit to celsius
        (temp_value - 32.00) * frac
    } else {
        //celsius to fahrenheit
        temp_value * frac + 32.00
    }
}
