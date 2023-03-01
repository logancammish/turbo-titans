use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use colored::Colorize;


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
    let car_option: [&str; 6] = [
        "one", "two", "three", "four", "five", "six"
    ];
    Songs.play_audio(Songs.get_file("engine-rev").as_str(), 4, true);
    for _ in 1..3 {
        println!("Welcome...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{}", ("Welcome to Turbo Titans!").green().underline());
    println!("You have 6 car options! Choose wisely...");
    for i in car_option {
        println!("{}", i);
    }
}
