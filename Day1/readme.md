# Rust Learning - Day 1 Exercises

## Overview
This document contains 4 exercises to practice fundamental Rust concepts including command-line arguments, error handling, control flow, and ownership/borrowing.

---

## Exercise 1: Mini-CLI "Echo & Sum"

### Objective
Modify `src/main.rs` to accept command-line arguments and process them accordingly.

### Requirements

#### Basic Functionality
- Accept command-line arguments
- Print arguments in format: `Args: ["arg1", "arg2", ...]`

#### Example Usage
```bash
cargo run -- hello chatgpt
# Output: Args: ["hello", "chatgpt"]
```

#### Number Summing
- If all arguments are numbers, calculate and print the sum
- Example: `cargo run -- 10 20 3` → `Sum = 33`

### Success Criteria
- ✅ No panic when mixing text and numbers
- ✅ Use `Result` for graceful error handling
- ✅ Display message for non-numeric values: "ข้ามค่าที่ไม่ใช่ตัวเลข: x"

### Hints
- Use `std::env::args()` - skip the first argument (program name)
- Use `str::parse::<i64>()` for parsing

---

## Exercise 2: BMI Calculator

### Objective
Create a BMI calculator that accepts weight and height as command-line arguments.

### Setup
```bash
cargo new bmi_calc
```

### Requirements

#### Input
- Weight in kilograms (kg)
- Height in meters (m)

#### Calculation
- BMI = weight / (height × height)
- Round result to 2 decimal places

#### BMI Categories
- < 18.5: Underweight
- 18.5 – 24.9: Normal
- 25.0 – 29.9: Overweight
- ≥ 30: Obese

### Success Criteria
- ✅ Check argument count - show usage if incomplete
- ✅ Handle parse errors and negative values gracefully
- ✅ Use `match` with `Result` for error handling
- ✅ Format decimals with `format!("{:.2}", value)`

### Example Usage
```bash
cargo run -- 70 1.75
# Output: BMI: 22.86 (Normal)
```

---

## Exercise 3: Ownership vs Borrowing

### Objective
Create a small file to experiment with ownership and borrowing concepts.

### Requirements

#### Functions to Implement
```rust
fn takes_ownership(s: String) {
    // Print the length of s
}

fn borrows(s: &String) {
    // Print the length of s without moving the value
}
```

#### Experiments
1. Call `takes_ownership(my_string)` then try to use `my_string` again → observe the error
2. Fix the code using borrowing (`&`)

### Success Criteria
- ✅ Clear understanding of the difference between "moving values" and "borrowing values"
- ✅ This is the core concept of Rust ownership system

### Key Learning Points
- **Ownership**: When you pass a value to a function, ownership is transferred
- **Borrowing**: When you pass a reference (`&`), you're borrowing the value temporarily
- **Move vs Borrow**: Moving transfers ownership permanently, borrowing allows temporary access

---

## Notes
- Focus on understanding error handling with `Result` type
- Practice proper argument parsing and validation
- Master the ownership system - it's fundamental to Rust programming