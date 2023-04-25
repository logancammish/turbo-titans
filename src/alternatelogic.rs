//unused code

use rand::prelude::*;

struct NPC {}
impl NPC {
    pub fn npc_move(car_location: &str, car_ypos: i32) -> &str {
        return "y";
    }
    pub fn append_npc_location(car_location: &str, npc_location: [String; 2], npc_ypos: [i32; 2]) -> &str {
        return "y";
    }
    pub fn get_winner(npc_ypos: [i32; 2], car_ypos: i32) -> bool {
        let mut Result = false;
        if (npc_ypos[0] < car_ypos) && (npc_ypos[1] < car_ypos) {
            Result = true;
        }
        return Result;
    }
}