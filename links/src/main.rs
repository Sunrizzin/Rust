fn main() {
    let s1 = String::from("Hello");
    let len = calculate_lenght(&s1);
    println!("Lenght '{}' equals {}", s1, len)
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}
