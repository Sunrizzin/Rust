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
/* Example visible from fn
    let r = String::from("hello");

    takes_ownership(r);
    println!("{}", r);
    let x = 5;
    makes_copy(5);
*/
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}