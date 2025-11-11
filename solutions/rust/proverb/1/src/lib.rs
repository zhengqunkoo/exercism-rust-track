pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    for i in 0..list.len() - 1 {
        result.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i], list[i + 1]
        ));
    }
    result.push_str(&format!(
        "And all for the want of a {}.",
        list[0]
    ));
    result
}