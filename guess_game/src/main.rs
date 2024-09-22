mod handling;
mod helpers;

use std::cmp::Ordering;
use handling::game::user_chose_num;
use handling::game::user_predict_num;
use rand::Rng;

pub fn main() {
    println!("#### Welcome to guess game #####");
    let rand_num = rand::thread_rng().gen_range(1..=user_chose_num());

    loop{
        let user_choice: i32 = user_predict_num();

        match user_choice.cmp(&rand_num) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break;
            },
        }
    }
}