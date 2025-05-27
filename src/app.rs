use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize, Color},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Padding},
    prelude::{Constraint, Layout},
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    bg_color: u8,
    fg_color: u8,
    sidebar_bg: u8,
    sidebar_fg: u8
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {

        self.bg_color = 233;
        self.fg_color = 147;

        self.sidebar_bg = 234;
        self.sidebar_fg = 147;

        while !self.exit {
            terminal.draw(|frame| {

                let title = Line::from(" File Manager ".bold());
                let sidebar_title = Line::from(" Files ".bold());


                let instructions = Line::from(vec![
                    " Quit ".into(),
                    "<Q> ".light_blue().bold(),
                ]);

                let block_bg = Block::bordered()
                    .title_bottom(instructions.centered())
                    .border_set(border::ROUNDED)
                    .padding(Padding::proportional(1))
                    .bg(Color::Indexed(self.bg_color))
                    .fg(Color::Indexed(self.fg_color));

                let block_sidebar = Block::bordered()
                    .border_set(border::ROUNDED)
                    .padding(Padding::proportional(1))
                    .bg(Color::Indexed(self.sidebar_bg))
                    .fg(Color::Indexed(self.sidebar_fg));


                let block = Paragraph::new("Hello World".bold())
                    .centered()
                    .block(Block::bordered()
                        .border_set(border::ROUNDED)
                        .padding(Padding::proportional(2))
                        .bg(Color::Indexed(self.bg_color))
                        .fg(Color::Indexed(self.fg_color))
                        .title(" Block "));

                let sidebar = Paragraph::new("Files")
                    .centered()
                    .block(Block::bordered()
                        .border_set(border::ROUNDED)
                        .padding(Padding::proportional(2))
                        .bg(Color::Indexed(self.sidebar_bg))
                        .fg(Color::Indexed(self.sidebar_fg))
                        .title(" Files "));


                let [left, right] = Layout::horizontal([Constraint::Percentage(50); 2])
                    .areas(frame.area());
                let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3])
                    .areas(right);

                frame.render_widget(block, right);
                frame.render_widget(sidebar, left);

            });
            self.handle_events()
        }
    }

    fn handle_events(&mut self){
        match event::read() {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
