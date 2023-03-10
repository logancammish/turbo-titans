/*
* TURBO TITANS
* LOGAN CAMMISH
*/

use std::{io::stdin};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use colored::Colorize;
use std::collections::HashMap;

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
        stdin().read_line(&mut input).expect("Error taking input. Please try again.");

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
    fn get_file(&self, input: &str) -> String { 
        return format!("C:/Users/l.j.cammish/Desktop/11 Digi TECH/rust/proj/project/src/audio/{}.wav", input); 
    }
    // play the given audio file
    // takes 3 arguments: location, len, and sleep
    // location is the location of the file, len is
    // the length it should sleep for, and sleep
    // tells the program whether or not it should
    // sleep the thread for the audio
    fn play_audio(&self, location: &str, len: u64, sleep: bool) -> &Songs {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap(); 
        let file: BufReader<File> = BufReader::new(File::open(location).unwrap()); /* bufreader is used to create a buffer,
                                                                                                * allowing for additional functionality 
                                                                                                * on the file read, but is inefficient*/
        let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap(); // decode the file
        stream_handle.play_raw(source.convert_samples()).expect("Error playing audio."); // attempt to play the audio file
        if sleep == true {
            std::thread::sleep(std::time::Duration::from_secs(len));
        }
        return self;
    }
}

fn main() { 
    // define the "car_options"  hashmap
    // a 'hashmap' is similar to dictionaries in other languages
    // both the key and its data are teh &str data type
    let car_options: HashMap<&str, &str> = HashMap::from([
        ("CARTYPE1", "a"),
        ("two", "a"),
        ("three", "a"),
        ("four", "a"),
        ("five", "a"),
        ("six", "a")
    ]);

    Songs.play_audio(Songs.get_file("engine-rev").as_str(), 4, true); // play the engine-rev sound
    println!("{}", ("Welcome to Turbo Titans!").green().underline()); // utilizes colored to print a colored output
    std::thread::sleep(std::time::Duration::from_secs(1)); // sleep the current thread
    println!("You have 6 car options! Choose wisely...");
    std::thread::sleep(std::time::Duration::from_secs(1/2));

    for (k,v ) in car_options.iter() {
        println!("{}: {}",k, v)
    }  // "iter" is used here to iterate through the hashmap.

    fn get_car() -> i32 {
        loop {
            let input: String = Game::take_input("i32");
            let input: &str = input.as_str().trim();
            if !(input == String::new()) {
                println!("You inputted: {}", input);
                return input.parse::<i32>().unwrap();
            } else {
                println!("Invalid input, please enter a value which can be converted to a 32 bit integer.")
            }
        }
    }
    fn check_car(input: i32) {
        match input {
            1 => {

            }
            2 => {

            }
            3 => {

            }
            4 => {

            }
            5  => {

            }
            6 => {

            }
            _ => {
                println!("Invalid input, please enter a value from 1-6");
                check_car(get_car());
            }
        }
    }

    check_car(get_car());

    Game::end();
}
