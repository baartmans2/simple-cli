#![feature(int_roundings)]

use std::{any::type_name, fmt::Display, io, str::FromStr};

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

fn check_min_max<T: PartialOrd + Display>(
    number: T,
    min_value: Option<T>,
    max_value: Option<T>,
) -> bool {
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

fn check_number_is_a_choice<T: PartialOrd + Display>(
    number: &T,
    choices: &Vec<T>,
    show_choices_on_failure: bool,
) -> bool {
    for choice in choices.iter() {
        if number == choice {
            return true;
        }
    }
    if show_choices_on_failure {
        print!("Your input ({}) is not an option of the choices: ", number);
        for choice in choices.iter() {
            print!("{}, ", choice);
        }
        print!("\n");
    } else {
        println!("Your input ({}) is not a valid choice.", number);
    }

    return false;
}

fn check_string_is_a_choice(
    input: &String,
    choices: &Vec<&str>,
    case_sensitive: bool,
    show_choices_on_failure: bool,
) -> bool {
    for choice in choices.iter() {
        if input == choice {
            return true;
        } else if input.to_lowercase() == choice.to_lowercase() && !case_sensitive {
            return true;
        }
    }
    if show_choices_on_failure {
        print!("Your input ({}) is not an option of the choices: ", input);
        for choice in choices.iter() {
            print!("{}, ", choice);
        }
        print!("\n");
    } else {
        print!("Your input ({}) is not a valid choice. ", input);
    }
    print!("(Case Sensitive: {})\n", case_sensitive);
    return false;
}

/// Displays a list of items.
///
/// # Arguments
///
/// * `header_message` - An option that can contain a string slice which holds a header message for the paginated list.
/// * `items` - An array of items of a type with 'Display' trait
///
/// # Example
///
/// ```
/// use simple_cli::*;
/// let items = vec!["Moe", "Larry", "Curly"];
/// print_list(Some("My list:"), &items);
///
/// ```
pub fn print_list<T: Display>(header_message: Option<&str>, items: &[T]) {
    print_prompt(header_message);
    for i in 0..items.len() {
        println!("{}", items[i])
    }
}

/// Clears all printed lines from the terminal.
///
/// # Example
///
/// ```no_run
/// use simple_cli::*;
/// clear_terminal();
/// ```
pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
}

/// Displays a paginated list of items.
///
/// # Arguments
///
/// * `header_message` - An option that can contain a string slice which holds a header message for the paginated list.
/// * `items` - An array of items of a type with 'Display' trait
/// * `items_per_page` - The number of items that will be displayed per page.
/// * `clear_on_update` - A boolean which denotes whether the terminal should clear each time the user navigates to a new page. This is helpful when making command-line apps that "re-render" a single display.
///
/// # Examples
///
/// ```no_run
/// use simple_cli::*;
/// let items = vec!["Moe", "Larry", "Curly"];
/// paginated_list(Some("Here is my paginated list:"), &items, 2, true);
///
/// use simple_cli::*;
/// let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// paginated_list::<i8>(Some("Here is my paginated list:"), &items, 2, true);
/// ```
pub fn paginated_list<T: Display>(
    header_message: Option<&str>,
    items: &[T],
    items_per_page: i32,
    clear_on_update: bool,
) {
    if items_per_page <= 0 {
        panic!("Items per page must be greater than zero.");
    }
    let mut quit = false;
    let number_of_items = items.len() as i32;
    let mut current_page: i32 = 1;
    let mut number_of_pages: i32 = number_of_items.div_ceil(items_per_page);
    if number_of_pages == 0 {
        number_of_pages = 1;
    }
    while !quit {
        print_prompt(header_message);
        let end_index: i32;
        if current_page == number_of_pages {
            end_index = number_of_items;
        } else {
            end_index = current_page * items_per_page;
        }
        if number_of_items > 0 {
            for i in ((current_page - 1) * items_per_page)..end_index {
                println!("{}", items[i as usize]);
            }
        }
        println!("(Page {} of {})", current_page, number_of_pages);
        let user_input = select_string_from_choices(
            Some("Press N to view the next page, P for previous, S for a specific page, or E to Exit."),
            Some("Press N to view the next page, P for previous, S for a specific page, or E to Exit."),
            vec!["N", "P", "S", "E"],
            false,
            true
        );
        match user_input.to_lowercase().as_str() {
            "n" => {
                if current_page < number_of_pages {
                    current_page += 1;
                }
            }
            "p" => {
                if current_page > 1 {
                    current_page -= 1;
                }
            }
            "s" => {
                current_page = select_number_from_choices(
                    Some("Enter the page you would like to view."),
                    Some("Enter the page you would like to view."),
                    (1..(number_of_pages + 1)).collect(),
                    false,
                );
            }
            "e" => {
                quit = true;
            }
            _ => {}
        }
        if clear_on_update {
            clear_terminal();
        }
    }
}

/// Prompts the user for a string input and returns it.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `repeat_message` - An option that can contain a string slice which holds a repeat message which will be displayed if the user enters invalid input
/// * `max_length` - An option that can contain a integer which specifies the maximum length the user's input can reach.
/// * `can_be_empty` - A boolean which denotes whether the user's input can be an empty string.
///
/// # Example
///
/// ```
/// use simple_cli::*;
/// let input = get_string(Some("Enter your name:"), Some("Enter your name:"), Some(25), false);
/// ```
pub fn get_string(
    prompt: Option<&str>,
    repeat_message: Option<&str>,
    max_length: Option<i32>,
    can_be_empty: bool,
) -> String {
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
        print_prompt(repeat_message);
    }
}

/// Prompts the user for a number input and returns it.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `repeat_message` - An option that can contain a string slice which holds a repeat message which will be displayed if the user enters invalid input
/// * `min_value` - An option that can contain a number of type T which specifies the minimum value the user can input.
/// * `max_value` - An option that can contain a number of type T which specifies the maximum value the user can input.
///
/// # Example
///
/// ```
/// use simple_cli::*;
/// let input = get_number::<i8>(Some("Enter an integer from 0 to 10:"), None, Some(0), Some(10));
///
/// let float_input = get_number::<f32>(Some("Enter a float from 0 to 10:"), None, Some(0.0), Some(10.0));
/// ```
pub fn get_number<T: PartialOrd + Display + FromStr + Copy>(
    prompt: Option<&str>,
    repeat_message: Option<&str>,
    min_value: Option<T>,
    max_value: Option<T>,
) -> T {
    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim().parse::<T>() {
                Ok(number) => {
                    if check_min_max(number, min_value, max_value) {
                        return number;
                    }
                }
                Err(_e) => {
                    println!("Please enter a valid {} value.", type_name::<T>());
                }
            },
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(repeat_message);
    }
}

/// Prompts the user to input a number from a selection of number choices, and returns the number the user selected. Panics if there are no numbers in the vector passed into the function.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `repeat_message` - An option that can contain a string slice which holds a repeat message which will be displayed if the user enters invalid input
/// * `choices` - A vector of numbers of type T which make up the choices the user can select from.
/// * `show_choices_on_failure` - Whether or not to show the available choices after invalid input.
///
/// # Example
///
/// ```
/// use simple_cli::*;
/// let choices: Vec<i8> = vec![1,2,3];
/// let choice = select_number_from_choices::<i8>(Some("Enter 1, 2 or 3"), None, choices, true);
///
/// ```
pub fn select_number_from_choices<T: PartialOrd + Display + FromStr + Copy>(
    prompt: Option<&str>,
    repeat_message: Option<&str>,
    choices: Vec<T>,
    show_choices_on_failure: bool,
) -> T {
    if choices.len() == 0 {
        panic!("You have not supplied a vector of at least one integer choices.")
    }

    print_prompt(prompt);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim().parse::<T>() {
                Ok(number) => {
                    if check_number_is_a_choice(&number, &choices, show_choices_on_failure) {
                        return number;
                    }
                }
                Err(_e) => {
                    println!("Please enter a valid {} value.", type_name::<T>());
                }
            },
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(repeat_message);
    }
}

/// Prompts the user to input a string from a selection of string choices, and returns the string the user selected. Panics if there are no strings in the choices vector passed into the function.
///
/// # Arguments
///
/// * `prompt` - An option that can contain a string slice which holds the prompt to present the user with.
/// * `repeat_message` - An option that can contain a string slice which holds a repeat message which will be displayed if the user enters invalid input
/// * `choices` - A vector of string slices which make up the choices the user can select from.
/// * `case_sensitive` - A boolean which represents whether the user's input is case-sensitive.
/// * `show_choices_on_failure` - Whether or not to show the available choices after invalid input.
///
/// # Example
///
/// ```
/// use simple_cli::*;
/// let choices = vec!["Moe", "Larry", "Curly"];
/// let choice = select_string_from_choices(Some("Select Moe, Larry, or Curly"), None, choices, false, true);
///
/// ```
pub fn select_string_from_choices(
    prompt: Option<&str>,
    repeat_message: Option<&str>,
    choices: Vec<&str>,
    case_sensitive: bool,
    show_choices_on_failure: bool,
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
                if check_string_is_a_choice(
                    &trimmed,
                    &choices,
                    case_sensitive,
                    show_choices_on_failure,
                ) {
                    return trimmed;
                }
            }
            Err(error) => panic!("Unexpected stdin error while reading input: {}", error),
        }
        input.clear();
        print_prompt(repeat_message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_prompt() {
        let no_prompt: Option<&str> = None;
        let some_prompt: Option<&str> = Some("Test Message.");
        assert_eq!(print_prompt(no_prompt), false);
        assert_eq!(print_prompt(some_prompt), true);
    }

    #[test]
    fn test_check_length() {
        let no_max_length: Option<i32> = None;
        let yes_max_length: Option<i32> = Some(10);
        let small_string = "hi";
        let big_string = "abcuiwehfuewnfiuewnf";
        assert_eq!(check_length(&small_string.len(), no_max_length), true);
        assert_eq!(check_length(&small_string.len(), yes_max_length), true);
        assert_eq!(check_length(&big_string.len(), yes_max_length), false);
    }

    #[test]
    fn test_check_empty() {
        let empty_string = "";
        let non_empty_string = "Hello!";
        assert_eq!(check_empty(&empty_string.len(), true), true);
        assert_eq!(check_empty(&empty_string.len(), false), false);
        assert_eq!(check_empty(&non_empty_string.len(), false), true);
        assert_eq!(check_empty(&non_empty_string.len(), true), true);
    }

    #[test]
    fn test_min_max() {
        let no_min: Option<i32> = None;
        let min: Option<i32> = Some(1);
        let no_max: Option<i32> = None;
        let max: Option<i32> = Some(3);
        let no_min_2: Option<f32> = None;
        let min_2: Option<f32> = Some(1.5);
        let no_max_2: Option<f32> = None;
        let max_2: Option<f32> = Some(3.5);
        assert_eq!(check_min_max(5, no_min, no_max), true);
        assert_eq!(check_min_max(-5, min, no_max), false);
        assert_eq!(check_min_max(-5, no_min, max), true);
        assert_eq!(check_min_max(5, no_min, max), false);
        assert_eq!(check_min_max(5, min, max), false);
        assert_eq!(check_min_max(2, min, max), true);
        assert_eq!(check_min_max(5.0, no_min_2, no_max_2), true);
        assert_eq!(check_min_max(-5.0, min_2, no_max_2), false);
        assert_eq!(check_min_max(-5.0, no_min_2, max_2), true);
        assert_eq!(check_min_max(5.0, no_min_2, max_2), false);
        assert_eq!(check_min_max(5.0, min_2, max_2), false);
        assert_eq!(check_min_max(2.0, min_2, max_2), true);
    }

    #[test]
    fn test_check_string_is_choice() {
        let choices = vec!["Earl", "Roger", "Mark"];
        let bob = String::from("Bob");
        let earl_uppercase = String::from("EARL");
        let mark = String::from("Mark");
        assert_eq!(check_string_is_a_choice(&bob, &choices, false, true), false);
        assert_eq!(check_string_is_a_choice(&bob, &choices, true, true), false);
        assert_eq!(
            check_string_is_a_choice(&earl_uppercase, &choices, false, true),
            true
        );
        assert_eq!(
            check_string_is_a_choice(&earl_uppercase, &choices, true, false),
            false
        );
        assert_eq!(check_string_is_a_choice(&mark, &choices, true, false), true);
    }

    #[test]
    fn test_check_num_is_choice() {
        let choices = vec![1, 5, 10, 15];
        let choices_float = vec![0.5, 1.5, 2.0, 3.35];
        assert_eq!(check_number_is_a_choice(&1, &choices, true), true);
        assert_eq!(check_number_is_a_choice(&5, &choices, false), true);
        assert_eq!(check_number_is_a_choice(&10, &choices, true), true);
        assert_eq!(check_number_is_a_choice(&15, &choices, false), true);
        assert_eq!(check_number_is_a_choice(&-50, &choices, true), false);
        assert_eq!(check_number_is_a_choice(&0.5, &choices_float, false), true);
        assert_eq!(check_number_is_a_choice(&1.5, &choices_float, true), true);
        assert_eq!(check_number_is_a_choice(&2.0, &choices_float, false), true);
        assert_eq!(check_number_is_a_choice(&3.35, &choices_float, true), true);
        assert_eq!(
            check_number_is_a_choice(&-5.5, &choices_float, false),
            false
        );
    }
}
