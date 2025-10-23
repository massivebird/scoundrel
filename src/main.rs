use self::deck::Deck;
use self::room::Room;
use crossterm::event::KeyCode;
use std::io::Write;

mod card;
mod deck;
mod room;

fn main() {
    let mut stdout = std::io::stdout();

    crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut rng = rand::rng();

    let mut deck = Deck::generate(&mut rng);

    let mut room: Room = Room::from_deck(&mut deck);

    while let Ok(event) = crossterm::event::read() {
        let Some(event) = event.as_key_event() else {
            continue;
        };

        match event.code {
            KeyCode::Char('q') => break,
            KeyCode::Char('w') => print!("ok"),
            KeyCode::Char('r') => {
                for card in room.iter() {
                    match card {
                        Some(card) => print!("{card} "),
                        None => print!("_ "),
                    }
                }
                println!("\r");
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
