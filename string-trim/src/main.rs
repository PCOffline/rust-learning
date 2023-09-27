fn main() {
    let test1 = "We need more spaces";
    assert_eq!(trim_spaces(test1), "We need more spaces");

    let test2 = String::from("   There's space in front");
    assert_eq!(trim_spaces(&test2), "There's space in front");

    let test3 = String::from("There's space to the rear   ");
    assert_eq!(trim_spaces(&test3), "There's space to the rear");

    let test4 = "   We're surrounded by space!   ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "      ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀  ";
    assert_eq!(trim_spaces(test7), "🚀")
}

fn trim_spaces(input: &str) -> &str {
    let bytes = input.as_bytes();
    let mut first_index_found = false;
    let mut first_index: usize = 0;
    let mut last_index: usize = 0;

    for (index, &item) in bytes.iter().enumerate() {
            if !first_index_found && item != b' ' {
                first_index = index;
                first_index_found = true;
            } else if item != b' ' {
                last_index = index + 1;
            }
    }

    &input[first_index..last_index]
}
