use std::io::{stdout, Write};
use rand::prelude::*;

pub struct Dice {}
impl Dice {
    pub fn generate() -> usize {
        println!("⚀⚁⚂⚃⚄⚅...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let die: [&str; 6] = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
        let rand_int: usize = rand::prelude::thread_rng().gen_range(0..5);
        println!("You rolled {}:\n{}\n", rand_int + 1, die[rand_int].trim());

        return rand_int;
    }
}



pub struct Car {}
impl Car {
    pub fn show(pos: &str, frame: i32) {
        fn flush() {
            stdout().flush().expect("Error flushing stdout");
        } 
        
        println!("-------------");
        println!("     ↑({})", frame);
        println!("-------------");

        print!("\x1B[2J\x1B[1;1H");
        match pos {
            "center" => {
                println!( "____ [.] ____");
                flush();
                return;
            }
            "right" => {
                println!( "_______ [.] _");
                flush();
                return;
            }
            "left" => {
                println!( "_ [.] _______"); 
                println!("__ [.] ______");
                flush();
                return;
            }
            _ => {
                println!("There was an error showing your car location.")
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_location() -> (i32, String) {
        return (1, String::from("center"));
    }
}

