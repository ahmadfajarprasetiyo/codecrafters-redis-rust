pub fn bulk_string(non_formated_str : &str) -> String {
    let res = "$".to_owned() + non_formated_str.len().to_string().trim();
    let res = res + "\r\n";
    let res = res + non_formated_str;
    let res = res + "\r\n";
    res
}

pub fn simple_string(non_formated_str : &str) -> String {
    let res = "+".to_owned() + non_formated_str;
    let res = res + "\r\n";
    res
}