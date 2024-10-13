use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use super::ListView;

pub(crate) struct Detail<'a> {
    list_view: &'a ListView<'a>,
}

impl<'a> Detail<'a> {
    pub(crate) fn new(list_view: &'a ListView) -> Self {
        Self { list_view }
    }
}

impl Widget for Detail<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let current_memo = self.list_view.get_current_memo();
        Paragraph::new(current_memo.content.clone().unwrap())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::proportional(1)),
            )
            .render(area, buf);
    }
}
