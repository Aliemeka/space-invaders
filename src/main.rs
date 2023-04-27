use std::error::Error;

use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("phew", "./sounds/phew.wav");
    audio.add("start", "./sounds/start.wav");
    audio.add("win", "./sounds/win.wav");

    audio.play("explode");
    audio.wait();

    Ok(())
}
