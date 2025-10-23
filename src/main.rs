use self::game::Game;
use crossterm::event::KeyCode;
use std::io::Write;

mod card;
mod deck;
mod game;
mod player;
mod room;

static WELCOME: &str = "Welcome to Scoundrel!\r";

static CONTROLS: &str = "\
[1 — 4]: Interact with card\r
[a]: Avoid room\r
[q]: Quit\r";

fn main() {
    let mut stdout = std::io::stdout();

    crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut rng = rand::rng();
    let mut game = Game::new(&mut rng);

    println!("{WELCOME}");
    println!("{CONTROLS}\n");

    game.print_game();

    while let Ok(event) = crossterm::event::read() {
        let Some(event) = event.as_key_event() else {
            continue;
        };

        match event.code {
            KeyCode::Char('1') => game.interact(0),
            KeyCode::Char('2') => game.interact(1),
            KeyCode::Char('3') => game.interact(2),
            KeyCode::Char('4') => game.interact(3),

            KeyCode::Char('a') => game.try_avoid(),

            KeyCode::Char('q') => break,
            _ => (),
        }

        stdout.flush().unwrap();
        println!("\r");
        game.print_game();

        if game.is_over() {
            println!("Game over!");
            break;
        }
    }
}
