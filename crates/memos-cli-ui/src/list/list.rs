use super::ListView;

use memos_api::models::V1Memo;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};

pub(crate) struct List<'a> {
    list_view: &'a ListView<'a>,
}

impl<'a> List<'a> {
    pub(crate) fn new(list_view: &'a ListView) -> Self {
        Self { list_view }
    }
}

impl Widget for List<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let padding_right = 2;
        let current_memo = self.list_view.get_current_memo();
        let mut lines = vec![];
        for memo in self.list_view.memos.iter() {
            let line = get_line(
                memo,
                (area.width - padding_right).into(),
                memo == current_memo,
            );
            lines.push(line);
            lines.push(Line::from(""));
        }
        let text = Text::from(lines);

        Paragraph::new(text)
            .block(Block::new().padding(Padding::new(0, padding_right, 2, 2)))
            .render(area, buf);
    }
}

fn get_line(memo: &V1Memo, width: usize, is_current: bool) -> Line {
    let snippet = memo.snippet.clone().unwrap();
    let snippet = get_snippet(&snippet).to_string();
    let max_len = if is_current { width - 3 } else { width };
    let snippet = if snippet.len() > max_len {
        format!("{} ...", &snippet[..max_len - 4])
    } else {
        snippet
    };
    let snippet = if is_current {
        let repeat = max_len - snippet.len();
        let append = &" ".repeat(repeat);
        format!("{snippet}{append} ->")
    } else {
        snippet
    };

    Line::from(snippet)
}

fn get_snippet(text: &str) -> &str {
    let text = &text[..text.find('\n').unwrap_or(text.len())];
    text
}
