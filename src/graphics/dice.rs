use rand::prelude::*;

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
    println!("You rolled {}:\n{}\r", rand_int + 1, die[rand_int].trim());
    
    return rand_int;
}
