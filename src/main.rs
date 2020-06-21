use owoify_rs::{Owoifiable, OwoifyLevel};

fn main() {
    let string: String = "Taiga is the best! Natsumi feels inferior to his brother, but he's a good boi as well!".into();
    println!("{}", string.owoify(&OwoifyLevel::Uvu));
}