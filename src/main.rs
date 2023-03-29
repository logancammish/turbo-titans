/*
* TURBO TITANS
* LOGAN CAMMISH
*/

use colored::Colorize;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{stdin, BufReader};
use std::process::Command;
use clearscreen::clear;
mod graphics;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde_json::{self, json};

mod string_funcs {
    pub fn is_num(input: &str) -> bool {
        let mut is = true;
        for i in input.trim().chars() {
            if !i.is_digit(10) {
                is = false;
            }
        }
        return is;
    }
}

struct Game;
impl Game {
    // end the game
    // this presents the user with the option to press "enter"
    // to close the application, then automatically closes it
    // when they press "enter"
    fn end() {
        println!("Game over! Press enter to exit the application.");
        let mut _temp: String = String::new();
        stdin().read_line(&mut _temp).expect("Error"); // wait for the user to press "enter"
        drop(_temp); // improves memory safety
    }
    fn take_input(output_type: &str) -> String {
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Error taking input. Please try again.");

        match output_type {
            "String" => {
                return input;
            }
            "i32" => {
                if !(string_funcs::is_num(input.as_str()) == false) {
                    return input;
                } else {
                    return String::new();
                }
            }
            _ => {
                Game::take_input(output_type);
                return String::new();
            }
        }
    }
}

struct Songs;
impl Songs {
    // get the file
    // this improves readability of the code so its not
    // necessary to constantly rewrite the location of
    // the audio files
    #[allow(dead_code)]
    fn get_file(&self, input: &str) -> String {
        return format!(
            "C:/Users/l.j.cammish/Downloads/turbo-titans-main/src/audio/{}.wav",
            input
        );
    }
    // play the given audio file
    // takes 3 arguments: location, len, and sleep
    // location is the location of the file, len is
    // the length it should sleep for, and sleep
    // tells the program whether or not it should
    // sleep the thread for the audio

    #[allow(dead_code)]
    fn play_audio(&self, location: &str, len: u64, sleep: bool) -> &Songs {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Error in OutputStream");
        let file: BufReader<File> =
            BufReader::new(File::open(location).expect("Error in BufReader")); /* bufreader is used to create a buffer,
                                                                                * allowing for additional functionality
                                                                                * on the file read, but is inefficient*/
        let source: Decoder<BufReader<File>> = Decoder::new(file).expect("Error in Decoder"); // decode the file
        stream_handle
            .play_raw(source.convert_samples())
            .expect("Error playing audio."); // attempt to play the audio file
        if sleep == true {
            std::thread::sleep(std::time::Duration::from_secs(len));
        }
        return self;
    }
}

fn main() {
    // define the "car_options"  array
    // an array is defined as [type; length] in rust
    // car_options is an array of string slices with a length of 6
    static CAR_OPTIONS: [&str; 6] = ["a", "a", "a", "a", "a", "a"];

    Songs.play_audio(Songs.get_file("engine-rev").as_str(), 4, true); // play the engine-rev sound
    println!("{}", ("GAME IS NOT READY AND SOME FEATURES MAY NOT BE FULLY FUNCTIONAL\nNOTE: MOVEMENT MECHNIC NOT FUNCTIONAL")
        .green()
        .underline()); // utilizes colored to print a colored output
    println!("{}", ("Welcome to Turbo Titans!").green().underline()); // utilizes colored to print a colored output
    std::thread::sleep(std::time::Duration::from_secs(1)); // sleep the current thread
    println!("You have 6 car options! Choose wisely..."); // intro
    std::thread::sleep(std::time::Duration::from_secs(1 / 2)); // "

    for k in 1..CAR_OPTIONS.len() {
        let v: &str = CAR_OPTIONS[k];
        println!("{}: {}", k, v)
    } // iterating through the array and printing its contents

    struct CarInput {}
    impl CarInput {
        fn get_car() -> usize {
            // get the chosen car
            loop {
                let input: String = Game::take_input("i32");
                let input: &str = input.as_str().trim();
                if !(input == String::new()) {
                    println!("You inputted: {}", input);
                    return input.parse::<usize>().unwrap();
                } else {
                    println!("Invalid input, please enter a value which can be converted to a 32 bit integer.")
                }
            }
        }
        fn check_car(input: usize) -> usize {
            // check if the car input was valid
            if !((input < 6) && (input > 0)) {
                println!("Invalid input, please enter a value from 1-6");
                return CarInput::check_car(CarInput::get_car()); // iterate
            } else {
                return input;
            }
        }
    }
    // oops...DRY failed
    struct LengthInput {}
    impl LengthInput {
        fn get_length() -> f64 {
            let input: String = Game::take_input("i32");
            let input: &str = input.as_str().trim();
            if !(input == String::new()) {
                println!("You inputted: {}", input);
                return input.parse::<f64>().unwrap();
            } else {
                println!("Invalid input, please enter a value which can be converted to a 32 bit integer.");
                return LengthInput::get_length();
            }
        }
        fn check_length(input: f64) -> f64 {
            // check if the car input was valid
            if !((input < 16.0) && (input > 4.0)) {
                println!("Invalid input, please enter a value from 5-15");
                return CarInput::check_car(CarInput::get_car()) as f64; // iterate
            } else {
                return input;
            }
        }
    }

    println!("Car choice: ");
    let car: f64 = CarInput::check_car(CarInput::get_car()) as f64; // get the users car!
    println!("Enter race length (5-15km): ");
    let length: f64 = LengthInput::check_length(LengthInput::get_length());
    println!("\n\n");

    let mut i: f64 = 0.0;
    loop {
        println!("\n{}km/{}km", i, length);
        if i > (length - 0.1) {
            break;
        }

        struct JSONRetrieve {}
        impl JSONRetrieve {
            fn get_f64_val(index: &str, playerinfo: serde_json::Value) -> f64 {
                let number: f64 = match json!(playerinfo.get(index)).as_f64() {
                    Some(n) => n,
                    None => 0.0,
                };
                return number;
            }
        }

        let dice_roll: f64 = graphics::Dice::generate(); // generate a dice roll
        let _playerinfo = std::io::BufReader::new(File::open("./playerinfo.json").expect("Error"));
        let playerinfo: serde_json::Value = serde_json::from_reader(_playerinfo).expect("Error");

        //println!("{:?}", JSONRetrieve::get_f64_val("highscore", playerinfo));
        //println!("{:?}", playerinfo);
        //println!("{}|{}", dice_roll, car);


        /*print!("Press enter to continue:");
        let mut temp: String = String::new();
        stdin().read_line(&mut temp).expect("Error");
        //drop(_temp); /* same code as earlier, except now it awaits for any input */
        println!("{}", temp);*/

        enable_raw_mode().expect("Error: Unable to enter raw mode, perhaps your Operating System is unsupported?");
        let read_line_cur: Event = read().unwrap();
        clear().expect("Err");
        std::thread::sleep(std::time::Duration::from_secs(1));
        if dice_roll == car {
            match read_line_cur {
                // match the input codes
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    i += 0.25;
                    graphics::Car::show("center", i);
                } // detect w key press

                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    i += 0.25;
                    graphics::Car::show("left", i);
                } // detect a key press

                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    i += 0.25;
                    graphics::Car::show("right", i);
                } // detect d key press

                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    break;
                } // detect esc key press

                _ => {
                    i += 0.25;
                } // no input
            }
        } else {
            match read_line_cur {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    break;
                }

                _ => {
                    i += 0.25;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        disable_raw_mode().expect("Error: Unable to exit raw mode, perhaps your Operating System is unsupported?");
    }
    Game::end();
}
