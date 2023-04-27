use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use rusty_audio::Audio;
use std::error::Error;

use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Set up audio
    let mut audio = Audio::new();
    audio.add("explode", "./sounds/explode.wav");
    audio.add("lose", "./sounds/lose.wav");
    audio.add("move", "./sounds/move.wav");
    audio.add("phew", "./sounds/phew.wav");
    audio.add("start", "./sounds/start.wav");
    audio.add("win", "./sounds/win.wav");

    // Play music at start
    audio.play("explode");

    // Game view Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Clean up
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
