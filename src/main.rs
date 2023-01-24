fn main() {
    println!(
        "Your input was: {}",
        simple_input::get_float(
            Some("Enter a floating point number."),
            Some(0.0),
            Some(100.0)
        )
    );
}
