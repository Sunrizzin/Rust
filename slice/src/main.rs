fn main() {
    let hello = String::from("Hello world!");

    let r = first_word(&hello);
    println!("{}", r);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}