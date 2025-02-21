use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let distinct_elements: HashSet<_> = input_str.split(",").collect();
    distinct_elements.len()
}
