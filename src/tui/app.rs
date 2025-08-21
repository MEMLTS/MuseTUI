use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::DefaultTerminal;

pub fn run(mut terminal: DefaultTerminal) -> anyhow::Result<()>{
    loop {
        terminal.draw(render)?;
        let event = event::read()?;
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}