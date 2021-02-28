use aoc09::Game;

const PLAYER_COUNT: usize = 491;
const FINAL_MARBLE: u32 = 71058;

fn main() {
    let mut game = Game::new(PLAYER_COUNT, FINAL_MARBLE);
    let high_score = game.play();
    println!("{}", high_score);

    let mut game = Game::new(PLAYER_COUNT, FINAL_MARBLE * 100);
    let high_score = game.play();
    println!("{}", high_score);
}
