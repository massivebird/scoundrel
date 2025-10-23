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

    'outer: loop {
        for event in crossterm::event::read() {
            let Some(event) = event.as_key_event() else {
                println!("con");
                continue;
            };

            match event.code {
                KeyCode::Char('q') => break 'outer,
                KeyCode::Char('w') => println!("w"),
                _ => (),
            }
        }
    }

    dbg!(room);
}
