use std::env;

fn main() {
    let args = env::args();
    let mut sum = 0;
    let mut numbers = Vec::new();
    let mut is_number = true;

    let continue_to_run = |arg: &String| -> Result<i32, String> {
        let num: i32 = arg
            .parse()
            .map_err(|_| format!("Failed to parse '{}' as integer", arg))?;
        Ok(num)
    };

    for arg in args.skip(1) {
        numbers.push(arg.clone());
        // Skip the first argument (program name)
        match continue_to_run(&arg) {
            Ok(num) => sum += num,
            Err(_) => {
                is_number = false;
                continue; // Skip this argument and continue with the next one
            }
        }
    }

    if is_number {
        println!("Sum: {}", sum);
    } else {
        println!("Numbers: {:?}", numbers);
    }
}
