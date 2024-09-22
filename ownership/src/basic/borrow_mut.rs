fn main(){
    let mut my_str: String = String::from("Hello ");
    push(&mut my_str);

    println!("{}",my_str);
}

fn push(val: &mut String){
    val.push_str("Rust")
}