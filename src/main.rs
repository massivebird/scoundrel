use std::io::Write;

use crossterm::event::KeyCode;

use self::{card::Card, deck::Deck};

mod card;
mod deck;

fn main() {
    let mut stdout = std::io::stdout();

    crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut rng = rand::rng();

    let mut deck = Deck::generate(&mut rng);

    let mut room: [Option<Card>; 4] = [const { None }; 4];

    // Initial populate
    for slot in &mut room {
        *slot = deck.draw();
    }

    while let Ok(event) = crossterm::event::read() {
        let Some(event) = event.as_key_event() else {
            continue;
        };

        match event.code {
            KeyCode::Char('q') => break,
            KeyCode::Char('w') => print!("ok"),
            KeyCode::Char('r') => {
                for card in room {
                    match card {
                        Some(card) => print!("{card} "),
                        None => print!("_ "),
                    }
                }
                println!("\r");
            },
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
