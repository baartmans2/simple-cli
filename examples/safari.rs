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
    simple_io::clear_terminal();
    simple_io::paginated_list::<&str>(
        Some("Animals seen on the Super Cool Safari:"),
        &animals,
        3,
        true,
    )
}
