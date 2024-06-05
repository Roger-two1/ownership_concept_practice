fn main() {
    let string_one = "Hi";
    let string_two = " there";
    let concatenated_string = concatenate_strings(&string_one, &string_two);
    println!("{}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}