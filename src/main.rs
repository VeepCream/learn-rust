use std::env;

fn bmi_calculator(weight: f64, height: f64) -> String {
    let m = height / 100.0;
    println!("Weight: {}, Height: {}", weight, height);
    let bmi = weight / (m * m);
    println!("BMI: {}", bmi);
    let category = match bmi {
        bmi if bmi < 18.5 => "Underweight",
        bmi if bmi < 24.9 => "Normal", 
        bmi if bmi < 29.9 => "Overweight",
        _ => "Obese",
    };
    category.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let sum = bmi_calculator(args[1].parse().unwrap(), args[2].parse().unwrap());
    
    println!("Sum = {}", sum);
}
