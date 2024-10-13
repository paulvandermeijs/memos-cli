mod detail;
mod list;

use anyhow::{Error, Result};
use crossterm::event::{self, KeyCode, KeyEventKind};
use memos_api::models::V1Memo;
use ratatui::prelude::*;
use ratatui::DefaultTerminal;
use std::cell::RefCell;
use std::{cmp, io};

use self::detail::Detail;
use self::list::List;

pub struct ListView<'a> {
    memos: &'a Vec<V1Memo>,
    current_index: RefCell<usize>,
}

impl<'a> ListView<'a> {
    pub fn try_new(value: &'a Vec<V1Memo>) -> Result<Self> {
        if value.len() < 1 {
            return Err(Error::msg("List is empty."));
        }

        let list_view = ListView {
            memos: value,
            current_index: 0.into(),
        };

        Ok(list_view)
    }

    pub fn draw(&self) -> io::Result<()> {
        let mut terminal = ratatui::init();

        terminal.clear()?;

        let result = self.run(terminal);

        ratatui::restore();

        result
    }

    fn run(&self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .constraints([Constraint::Min(2)])
                    .horizontal_margin(20)
                    .vertical_margin(10)
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(50)])
                    .split(frame.area());

                let list = List::new(&self);
                let detail = Detail::new(&self);

                frame.render_widget(list, layout[0]);
                frame.render_widget(detail, layout[1]);
            })?;

            if let event::Event::Key(key) = event::read()? {
                match (key.kind, key.code) {
                    (KeyEventKind::Press, KeyCode::Char('j')) => {
                        let current_index = *self.current_index.borrow();
                        let current_index = current_index.saturating_add(1);
                        let current_index = cmp::min(self.memos.len() - 1, current_index);
                        self.current_index.replace(current_index);
                    }
                    (KeyEventKind::Press, KeyCode::Char('k')) => {
                        let current_index = *self.current_index.borrow();
                        let current_index = current_index.saturating_sub(1);
                        self.current_index.replace(current_index);
                    }
                    (KeyEventKind::Press, KeyCode::Char('q')) => return Ok(()),
                    _ => (),
                }
            }
        }
    }

    fn get_current_memo(&self) -> &V1Memo {
        let current_index = *self.current_index.borrow();
        &self.memos[current_index]
    }
}
