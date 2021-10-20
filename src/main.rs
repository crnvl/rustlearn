fn main() {
    let borrow_str = String::from("something");
    let length = borrow_a_str(&borrow_str);

    println!("{} is {} chars long!", borrow_str, length)
}

fn borrow_a_str(s: &String) -> usize {
    s.len()
}
