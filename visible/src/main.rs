fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

/* Example howto visible works
    let s1 = String::from("Alex");
    let s2 = s1;
    println!("{}", s1);
*/
/* Example howto clone works
    let s1 = String::from("Alex");
    let mut s2 = s1.clone();
    s2.push_str(" Usanov");
    println!("{} | {}", s1, s2);
*/
}
