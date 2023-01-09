fn compare_number(first_roman: &str, second_roman: &str) -> bool {
    return if get_roman_value(first_roman) < get_roman_value(second_roman) {
        true
    } else {
        false
    }
}

fn get_roman_value(roman: &str) -> i32 {
    let mut value_num: i32 = 0;

    for c in roman.chars() {
        match c {
            'M' => value_num += 1000,
            'D' => value_num += 500,
            'C' => value_num += 100,
            'L' => value_num += 50,
            'X' => value_num += 10,
            'V' => value_num += 5,
            'I' => value_num += 1,
            _ => {},
        }
    }

    value_num
}


fn main() {
    println!("{}", compare_number("I", "I"));
    println!("{}", compare_number("I", "II"));
    println!("{}", compare_number("II", "I"));
    println!("{}", compare_number("V", "IIIII"));
    println!("{}", compare_number("MDCLXV", "MDCLXVI"));
    println!("{}", compare_number("MM", "MDCCCCLXXXXVIIII"));
    println!("{}", compare_number("MM", ""));
}