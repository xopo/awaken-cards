// use crate::app::App;
use ratatui::crossterm::event::{self, Event};

use crate::app::App;

mod app;
mod data;
mod models;
mod ui;

fn main() -> anyhow::Result<()> {
    let flash_cards = data::load_cards()?;
    let mut app = App::new(flash_cards);

    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()?
            && app.handle_key(key)
        {
            println!("exit");
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
