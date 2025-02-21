pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.split('(').collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }
    let num_in_base_str = parts[0].trim();
    let from_base: u32 = parts[1].trim_end_matches(')').parse().expect("Invalid base");

    let num_in_base10 = u32::from_str_radix(num_in_base_str, from_base).expect("Invalid number in the input base");

    if to_base < 2 || to_base > 16 {
        panic!("Base out of range.");
    }

    if num_in_base10 == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut num = num_in_base10;

    let digits = "0123456789abcdef";

    while num > 0 {
        let remainder = (num % to_base) as usize;
        result.push(digits.chars().nth(remainder).unwrap());
        num /= to_base;
    }

    result.chars().rev().collect()
}