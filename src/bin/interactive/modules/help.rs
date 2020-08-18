use crate::interactive::modules::{Module, HandleInput, Draw};
use bookmark_lib::Registry;
use tui::backend::Backend;
use crate::interactive::table::StatefulTable;
use crate::interactive::interface::{InputMode, SuppressedAction};
use crate::interactive::url_table_item::URLItem;
use std::error::Error;
use termion::event::Key;
use tui::Frame;
use crate::interactive::widgets::rect::centered_rect;
use tui::widgets::{Paragraph, Clear, Block, Borders};
use tui::style::{Style, Color, Modifier};
use tui::layout::Alignment;
use tui::text::{Span, Spans};

pub(crate) struct HelpPanel {}

impl<R: Registry, B: Backend> Module<R, B> for HelpPanel {}

impl<R: Registry> HandleInput<R> for HelpPanel {
    fn handle_input(&mut self, input: Key, _registry: &R, _table: &mut StatefulTable<URLItem>) -> Result<InputMode, Box<dyn Error>> {
        match input {
            Key::Esc | Key::Char('\n') | Key::Char('h') => {
                return Ok(InputMode::Normal);
            }
            Key::Char('q') => {
                return Ok(InputMode::Normal);
            }
            _ => {}
        }

        return Ok(InputMode::Suppressed(SuppressedAction::ShowHelp)) // TODO: consider returning Option<InputMode>
    }
}

impl<B: Backend> Draw<B> for HelpPanel {
    fn draw(&self, mode: InputMode, f: &mut Frame<B>) {
        if mode == InputMode::Suppressed(SuppressedAction::ShowHelp) {
            self.show_help_popup(f);
        }
    }
}

impl HelpPanel {

    pub fn new() -> HelpPanel {
        return HelpPanel{}
    }

    fn show_help_popup<'a, B: Backend>(&self, f: &mut Frame<B>) {
        let text = vec![
            Spans::from("'ENTER'            - open bookmarked URL"),
            Spans::from("'/' or 'CTRL + F'  - search for URLs"),
            Spans::from("'d'                - delete URL"),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::LightBlue))
            .title(Span::styled(
                "Help - press ESC to close".to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ));

        let area = centered_rect(60, 40, f.size());
        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

}
