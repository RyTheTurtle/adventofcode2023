/**
 * Structs for representing the games in day 2
 */
#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    pub red_count: u32,
    pub green_count: u32,
    pub blue_count: u32,
}
pub struct GameCubeCount(pub u32, pub u32, pub u32);
