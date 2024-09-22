fn main() {
    let my_str = String::from("Hello Rust");
    let new_str = my_str;

    // print!("{}", my_str) // Error: borrow of moved value: `my_str`
    print!("{}", new_str)
}