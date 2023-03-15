use rand::prelude::*;

pub struct Dice {}
impl Dice {
    pub fn generate() -> usize {
        let die: [&str; 6] = [
            "__________\n|        |\n|    *   |\n|        |\n|________|",
            "__________\n|        |\n|  *  *  |\n|        |\n|________|",
            "__________\n|        |\n|  *  *  |\n|    *   |\n|________|",
            "__________\n|        |\n|  *  *  |\n|  *  *  |\n|________|",
            "__________\n|        |\n| * * *  |\n| *   *  |\n|________|",
            "__________\n| *   *  |\n| *   *  |\n| *   *  |\n|________|",
        ];
        let rand_int: usize = rand::prelude::thread_rng().gen_range(0..5);
        println!("You rolled {}:\n{}\n", rand_int + 1, die[rand_int].trim());

        return rand_int;
    }
}



pub struct Car {}
impl Car {
    pub fn show(pos: &str) {
        match pos {
            "left" => {
                println!( "____ [.] ____");
                return;
            }
            "right" => {
                println!( "_______ [.] _");
                return;
            }
            "center" => {
                println!( "_ [.] _______");
                return;
            }
            _ => {
                println!("There was an error showing your car location.")
            }
        }
    }

    pub fn get_location() -> (i32, String) {
        return (1, String::from("center"));
    }
}