use std::io;

fn main() {

    // Instantiate the variable that will hold our temp
    let mut temp = String::new();
    let mut unit = String::new();

    // Input our Temperature
    println!("Please tell me what is the temperature we are converting:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    // Convert our input to a float temp
    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };


    // Read in and Convert our Units to Char
    println!("What are the units? (F or C)");
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line.");
    let unit: char = unit.trim().chars().next()
        .expect("Passed string wasnt a Char");

    // Conditional switch to sawp between temps
    if unit == 'C' {
        let converted_temp: f32 = convert_c_to_f(temp);
        println!("Here is your converted temp: {converted_temp}F");
    } else if unit == 'F' {
        let converted_temp: f32 = convert_f_to_c(temp);
        println!("Here is your converted temp: {converted_temp}C");
    } else {
        println!("Incorrect units passed!");
        return;
    }
}

fn convert_c_to_f(temp: f32) -> f32 {
    // Convert units from C to F
    let factor: f32 = 9.0 / 5.0;
    return (temp * factor) + 32.0
}

fn convert_f_to_c(temp: f32) -> f32 {
    // Convert units from F to C
    let factor: f32 = 5.0 / 9.0;
    return (temp - 32.0) * factor
}
