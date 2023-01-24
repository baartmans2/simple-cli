use std::io;

fn print_prompt(prompt: Option<&str>) -> bool {
    match prompt {
        Some(input_prompt) => {
            println!("{}", input_prompt);
            return true;
        }
        None => {
            return false;
        }
    }
}

fn check_length(length: &usize, max_length: Option<i32>) -> bool {
    match max_length {
        Some(max) => {
            let input_length = *length as i32;
            if input_length > max {
                println!(
                    "Your input is {} characters higher than the {} character limit. Please try again.",
                    input_length - max,
                    length
                );
                return false;
            } else {
                return true;
            }
        }
        None => return true,
    }
}

fn check_empty(length: &usize, can_be_empty: bool) -> bool {
    let input_length = *length as i32;
    if input_length <= 0 && !can_be_empty {
        println!("Your input cannot be empty.");
        return false;
    } else {
        return true;
    }
}

fn check_min_max_float(number: f32, min_value: Option<f32>, max_value: Option<f32>) -> bool {
    match min_value {
        Some(min) => {
            if number < min {
                println!(
                    "Your input ({}) is lower than the minimum allowed value of {}.",
                    number, min
                );
                return false;
            }
        }
        None => {}
    }
    match max_value {
        Some(max) => {
            if number > max {
                println!(
                    "Your input ({}) is larger than the maximum allowed value of {}.",
                    number, max
                );
                return false;
            }
        }
        None => {}
    }
    return true;
}

fn check_min_max_integer(number: i32, min_value: Option<i32>, max_value: Option<i32>) -> bool {
    match min_value {
        Some(min) => {
            if number < min {
                println!(
                    "Your input ({}) is lower than the minimum allowed value of {}.",
                    number, min
                );
                return false;
            }
        }
        None => {}
    }
    match max_value {
        Some(max) => {
            if number > max {
                println!(
                    "Your input ({}) is larger than the maximum allowed value of {}.",
                    number, max
                );
                return false;
            }
        }
        None => {}
    }
    return true;
}

fn check_int_is_a_choice(number: &i8, choices: &Vec<i8>) -> bool {
    for choice in choices.iter() {
        if number == choice {
            return true;
        }
    }
    print!(
        "Your selection ({}) was not an option of the choices: ",
        number
    );
    for choice in choices.iter() {
        print!("{}, ", choice);
    }
    print!("\n");
    return false;
}

fn check_string_is_a_choice(input: &String, choices: &Vec<&str>, case_sensitive: bool) -> bool {
    for choice in choices.iter() {
        if input == choice {
            return true;
        } else if input.to_lowercase() == choice.to_lowercase() && !case_sensitive {
            return true;
        }
    }
    print!(
        "Your selection ({}) was not an option of the choices: ",
        input
    );
    for choice in choices.iter() {
        print!("[{}] ", choice);
    }
    print!("(Case Sensitive: {})\n", case_sensitive);
    return false;
}

/// Prompts the user for a string input and returns it.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `max_length` - An option that can contain a integer which specifies the maximum length the user's input can reach.
/// * `can_be_empty` - A boolean which denotes whether the user's input can be an empty string.
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_string(prompt: Option<&str>, max_length: Option<i32>, can_be_empty: bool) -> String {
    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let trimmed_input = input.trim();
                let length = trimmed_input.len();
                if check_length(&length, max_length) && check_empty(&length, can_be_empty) {
                    return trimmed_input.to_string();
                }
            }
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(prompt);
    }
}

/// Prompts the user for an integer input and returns it.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `min_value` - An option that can contain a integer which specifies the minimum value the user can input.
///  * `max_value` - An option that can contain a integer which specifies the maximum value the user can input.
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_int(prompt: Option<&str>, min_value: Option<i32>, max_value: Option<i32>) -> i32 {
    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim().parse::<i32>() {
                Ok(number) => {
                    if check_min_max_integer(number, min_value, max_value) {
                        return number;
                    }
                }
                Err(_e) => {
                    println!(
                        "Please enter a valid integer between {} and {}.",
                        i32::MIN,
                        i32::MAX
                    );
                }
            },
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(prompt);
    }
}

/// Prompts the user for a floating point number input and returns it.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `min_value` - An option that can contain a floating point number which specifies the minimum value the user can input.
///  * `max_value` - An option that can contain a floating point number which specifies the maximum value the user can input.
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_float(prompt: Option<&str>, min_value: Option<f32>, max_value: Option<f32>) -> f32 {
    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim().parse::<f32>() {
                Ok(number) => {
                    if check_min_max_float(number, min_value, max_value) {
                        return number;
                    }
                }
                Err(_e) => {
                    println!(
                        "Please enter a valid floating point number between {} and {}.",
                        f32::MIN,
                        f32::MAX
                    );
                }
            },
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(prompt);
    }
}

/// Prompts the user to input an integer from a selection of integer choices, and returns the integer the user selected. Panics if there are no integers in the choices vector passed into the function.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `choices` - A vector of integers which make up the choices the user can select from.
///
/// # Examples
///
/// ```
///
/// ```
pub fn select_int_from_choices(prompt: Option<&str>, choices: Vec<i8>) -> i8 {
    if choices.len() == 0 {
        panic!("You have not supplied a vector of at least one integer choices.")
    }

    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim().parse::<i8>() {
                Ok(number) => {
                    if check_int_is_a_choice(&number, &choices) {
                        return number;
                    }
                }
                Err(_e) => {
                    println!(
                        "Please enter a valid integer between {} and {}.",
                        i8::MIN,
                        i8::MAX
                    );
                }
            },
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(prompt);
    }
}

/// Prompts the user to input a string from a selection of string choices, and returns the string the user selected. Panics if there are no strings in the choices vector passed into the function.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `choices` - A vector of string slices which make up the choices the user can select from.
///  * `case_sensitive` - A boolean which represents whether the user's input is case-sensitive.
///
/// # Examples
///
/// ```
///
/// ```
pub fn select_string_from_choices(
    prompt: Option<&str>,
    choices: Vec<&str>,
    case_sensitive: bool,
) -> String {
    if choices.len() == 0 {
        panic!("You have not supplied a vector of at least one string choices.")
    }
    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let trimmed = input.trim().to_string();
                if check_string_is_a_choice(&trimmed, &choices, case_sensitive) {
                    return trimmed;
                }
            }
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(prompt);
    }
}

fn run_unit_tests() {
    test_print_prompt();
    test_check_length();
    test_check_empty();
    test_min_max_float();
    test_min_max_int();
    test_check_string_is_choice();
    test_check_int_is_choice();
    println!("All unit tests have passed.");
}

fn test_print_prompt() {
    println!("Testing prompt printing...");
    let no_prompt: Option<&str> = None;
    let some_prompt: Option<&str> = Some("Test Message.");
    assert_eq!(print_prompt(no_prompt), false);
    println!("Test #1 passed.");
    assert_eq!(print_prompt(some_prompt), true);
    println!("Test #2 passed.");
    println!("Prompt printing tests passed.")
}

fn test_check_length() {
    println!("Testing check_length function...");
    let no_max_length: Option<i32> = None;
    let yes_max_length: Option<i32> = Some(10);
    let small_string = "hi";
    let big_string = "abcuiwehfuewnfiuewnf";
    assert_eq!(check_length(&small_string.len(), no_max_length), true);
    println!("Test #1 passed.");
    assert_eq!(check_length(&small_string.len(), yes_max_length), true);
    println!("Test #2 passed.");
    assert_eq!(check_length(&big_string.len(), yes_max_length), false);
    println!("Test #3 passed.");
    println!("All check_length function tests passed.");
}

fn test_check_empty() {
    println!("Testing check_empty function...");
    let empty_string = "";
    let non_empty_string = "Hello!";
    assert_eq!(check_empty(&empty_string.len(), true), true);
    println!("Test #1 passed.");
    assert_eq!(check_empty(&empty_string.len(), false), false);
    println!("Test #2 passed.");
    assert_eq!(check_empty(&non_empty_string.len(), false), true);
    println!("Test #3 passed.");
    assert_eq!(check_empty(&non_empty_string.len(), true), true);
    println!("Test #4 passed.");
    println!("All check_empty function tests passed.");
}

fn test_min_max_float() {
    let no_min: Option<f32> = None;
    let min: Option<f32> = Some(1.4);
    let no_max: Option<f32> = None;
    let max: Option<f32> = Some(3.13);
    println!("Testing check_min_max_float function...");
    assert_eq!(check_min_max_float(5.312, no_min, no_max), true);
    println!("Test #1 passed.");
    assert_eq!(check_min_max_float(-5.32, min, no_max), false);
    println!("Test #2 passed.");
    assert_eq!(check_min_max_float(-5.32, no_min, max), true);
    println!("Test #3 passed.");
    assert_eq!(check_min_max_float(5.312, no_min, max), false);
    println!("Test #4 passed.");
    assert_eq!(check_min_max_float(5.312, min, max), false);
    println!("Test #5 passed.");
    assert_eq!(check_min_max_float(3.01, min, max), true);
    println!("Test #6 passed.");
    println!("All check_min_max_float function tests passed.");
}

fn test_min_max_int() {
    let no_min: Option<i32> = None;
    let min: Option<i32> = Some(1);
    let no_max: Option<i32> = None;
    let max: Option<i32> = Some(3);
    println!("Testing check_min_max_integer function...");
    assert_eq!(check_min_max_integer(5, no_min, no_max), true);
    println!("Test #1 passed.");
    assert_eq!(check_min_max_integer(-5, min, no_max), false);
    println!("Test #2 passed.");
    assert_eq!(check_min_max_integer(-5, no_min, max), true);
    println!("Test #3 passed.");
    assert_eq!(check_min_max_integer(5, no_min, max), false);
    println!("Test #4 passed.");
    assert_eq!(check_min_max_integer(5, min, max), false);
    println!("Test #5 passed.");
    assert_eq!(check_min_max_integer(2, min, max), true);
    println!("Test #6 passed.");
    println!("All check_min_max_integer function tests passed.");
}

fn test_check_string_is_choice() {
    let choices = vec!["Earl", "Roger", "Mark"];
    let bob = String::from("Bob");
    let earl_uppercase = String::from("EARL");
    let mark = String::from("Mark");
    println!("Testing check_string_is_choice function...");
    assert_eq!(check_string_is_a_choice(&bob, &choices, false), false);
    println!("Test #1 passed.");
    assert_eq!(check_string_is_a_choice(&bob, &choices, true), false);
    println!("Test #2 passed.");
    assert_eq!(
        check_string_is_a_choice(&earl_uppercase, &choices, false),
        true
    );
    println!("Test #3 passed.");
    assert_eq!(
        check_string_is_a_choice(&earl_uppercase, &choices, true),
        false
    );
    println!("Test #4 passed.");
    assert_eq!(check_string_is_a_choice(&mark, &choices, true), true);
    println!("Test #5 passed.");

    println!("All check_string_is_choice function tests passed.");
}

fn test_check_int_is_choice() {
    let choices = vec![1, 5, 10, 15];
    println!("Testing check_int_is_choice function...");
    assert_eq!(check_int_is_a_choice(&1, &choices), true);
    println!("Test #1 passed.");
    assert_eq!(check_int_is_a_choice(&5, &choices), true);
    println!("Test #2 passed.");
    assert_eq!(check_int_is_a_choice(&10, &choices), true);
    println!("Test #3 passed.");
    assert_eq!(check_int_is_a_choice(&15, &choices), true);
    println!("Test #4 passed.");
    assert_eq!(check_int_is_a_choice(&-50, &choices), false);
    println!("Test #5 passed.");
    println!("All check_int_is_choice function tests passed.");
}
