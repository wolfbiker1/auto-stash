#[cfg(feature = "termion")]
pub mod event;

use tui::widgets::ListState;
use tui::text::{Span, Spans};
use tui::style::{Color, Modifier, Style};
use diff::LineDifference;
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub fn process_new_version(diffs: Vec<LineDifference>) -> Vec<Spans<'static>> {
    let mut v: Vec<Span> = vec![];
    let mut spans: Vec<Spans> = vec![];
    for diff in &diffs {
        v.push(Span::raw("\n"));
        v.push(Span::styled(diff.line_number.to_string(), Style::default().fg(Color::Blue)));
        v.push(Span::raw("->"));
        v.push(Span::styled(diff.line.clone(), Style::default().fg(Color::Red)));
        v.push(Span::raw("->"));
        v.push(Span::styled(diff.changed_line.clone(), Style::default().fg(Color::Green)));
        v.push(Span::raw("\n"));
        spans.push(
            Spans::from(v.clone())
        );
        v.clear();
    }
    spans
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
