//To run this example `cargo run --example favorite_color --release`

fn main() {
    let colors: Vec<&str> = vec![
        "Blue", "Red", "Green", "Yellow", "Orange", "Purple", "Brown", "Pink", "Gray", "Black",
        "White",
    ];
    simple_cli::clear_terminal();
    println!(
        "Your favorite color is {}!",
        simple_cli::select_string_from_choices(
            Some("Enter your favorite color!"),
            Some("That isn't a color!"),
            colors,
            false,
            false,
        )
    );
}
