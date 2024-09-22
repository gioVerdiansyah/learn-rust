 fn main(){
    let my_str: String = String::from("Hello Rust");
    let new_str: &String = &my_str; // borrow ownership

    println!("{}",my_str);
    println!("{}",new_str)
}