

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use colored::Colorize;
use std::collections::HashMap;

struct Songs;
impl Songs {
    fn get_file(&self, input: &str) -> String { 
        return format!("./audio/{}.wav", input); 
    }
    fn play_audio(&self, location: &str, len: u64, sleep: bool) -> &Songs {
        // this portion was modified from https://docs.rs/rodio/latest/rodio/
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(location).unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).expect("Error playing audio.");
        //
        if sleep == true {
            std::thread::sleep(std::time::Duration::from_secs(len))
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

}
