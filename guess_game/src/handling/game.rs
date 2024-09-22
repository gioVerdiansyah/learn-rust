use crate::helpers::input::typing;

pub fn user_chose_num() -> i32 {
    loop {
        let input: String = typing("Please input range (1-?): ");
        let input = input.trim();

        match input.parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Input not valid! Please try Again!")
        }
    }
}

pub fn user_predict_num() -> i32 {
    loop {
        let input: String = typing("Your number: ");
        let input = input.trim();

        match input.parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Input not valid! Please try Again!")
        }
    }
}