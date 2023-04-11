
use rand::prelude::*;

pub struct Dice {}
impl Dice {
    pub fn generate() -> f64 {
        println!("⚀⚁⚂⚃⚄⚅...");
        std::thread::sleep(std::time::Duration::from_secs(1)); // sleep thread
        let die: [&str; 6] = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"]; // 6 length string slice array
        let rand_int: usize = rand::prelude::thread_rng().gen_range(0..5); // random integer from 0 to 5
        println!("You rolled {}:\n{}\n", rand_int + 1, die[rand_int].trim()); // you rolled...

        return 5.0; // (rand_int + 1) as f64; // return the random integer generated
    }
}



pub struct Car {}
impl Car {
    pub fn show(pos: &str, _frame: f64) {
         /*println!("-------------");
        println!("     ↑({})", frame);
        println!("-------------");*/

        print!("\x1B[2J\x1B[1;1H");
        match pos {
            "center" => {
                println!( "____ [.] ____");
                return;
            }
            "right" => {
                println!( "_______ [.] _");
                return;
            }
            "left" => {
                println!( "_ [.] _______");
                println!("__ [.] ______");
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
