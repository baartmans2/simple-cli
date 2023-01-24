//To run this example `cargo run --example safari --release`

fn main() {
    let animals = vec![
        "Hippo",
        "Elephant",
        "Lion",
        "Crocodile",
        "Giraffe",
        "Cheetah",
        "Hyena",
        "Rhino",
        "Buffalo",
        "Gorilla",
        "Mongoose",
        "Impala",
        "Mosquito",
        "Bird",
    ];
    simple_cli::clear_terminal();
    simple_cli::paginated_list::<&str>(
        Some("Animals seen on the Super Cool Safari:"),
        &animals,
        3,
        true,
    )
}
