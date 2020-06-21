use owoify_rs::{Owoifiable, OwoifyLevel};

fn main() {
    let string: String = "Camp Buddy is a story about courage and love.".into();
    println!("{}", string.owoify(&OwoifyLevel::Uvu));
}