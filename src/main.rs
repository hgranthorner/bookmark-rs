use crossterm::event::{poll, read, Event};
use std::io;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            terminal.draw(|mut f| {
                let size = f.size();
                let block = Block::default().title("Block").borders(Borders::ALL);
                f.render_widget(block, size);
            });
        }
    }
}
