use fraction::Fraction;

fn main() {
    // Create fractions 5/6 and 1/13
    let fraction1 = Fraction::new(5u32, 6u32);
    let fraction2 = Fraction::new(1u32, 13u32);

    // Calculate the sum
    let sum = fraction1 + fraction2;

    // Output the result to the console
    println!("5/6 + 1/13 = {}", sum);
}
