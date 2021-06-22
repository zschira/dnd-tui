use crate::app::{Component, StatefulComponent};
use crate::components::{Level, SearchBar,
                        SearchResults, SelectState, StatefulList};

use tui::layout::{Alignment, Rect};
use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use std::borrow::Cow;


impl<'a, B: Backend> Component<B> for SearchBar<'a> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let paragraph = Paragraph::new(Spans::from(vec![
                Span::raw(self.value.borrow().value.clone())
            ]))
            .block(
                Block::default()
                .title(self.name)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(
                        match self.selected {
                            SelectState::None => Color::Gray,
                            SelectState::Highlighted => Color::Blue,
                            SelectState::Selected => Color::Yellow,
                        }
                    )
                )
            )
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }
}

impl<'a, B: Backend> Component<B> for Level<'a> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let paragraph = Paragraph::new(Spans::from(vec![
                Span::raw(match self.level {
                        1..=19 => self.level.to_string(),
                        _ => String::from("Any"),
                    }
                )
            ]))
            .block(
                Block::default()
                .title(self.name)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(
                        match self.selected {
                            SelectState::None => Color::Gray,
                            SelectState::Highlighted => Color::Blue,
                            SelectState::Selected => Color::Yellow,
                        }
                    )
                )
            )
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }
}

impl<'a, T, B> Component<B> for StatefulList<'a, T>
where
    T: Into<Cow<'static, str>> + Clone, B: Backend
{
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.clone()))]))
            .collect();

        let items = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(self.name)
                .border_style(Style::default().fg(
                        match self.selected {
                            SelectState::None => Color::Gray,
                            SelectState::Highlighted => Color::Blue,
                            SelectState::Selected => Color::Yellow,
                        }
                    )
                )
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(items, area, &mut self.state.clone());
    }
}

impl<'a, T: 'static + Clone> SearchResults<T>
where ListItem<'a>: From<T>
{
    fn list(&self) -> List<'a> {
        let items: Vec<ListItem> = self.items
            .borrow_mut()
            .iter()
            .map(|i| Into::<ListItem>::into(i.clone()))
            .collect();

        let items = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Search Results")
                .border_style(Style::default().fg(
                        match self.selected {
                            SelectState::None => Color::Gray,
                            SelectState::Highlighted => Color::Blue,
                            SelectState::Selected => Color::Yellow,
                        }
                    )
                )
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        items
    }
}

impl<'a, B: Backend, T: 'static + Clone> Component<B> for SearchResults<T>
where T: Component<B>, ListItem<'a>: From<T>
{
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        if !self.item_view {
            let list_items = self.list();
            f.render_stateful_widget(list_items, area, &mut self.state.clone());
        } else {
            if let Some(i) = self.state.selected() {
                self.items.borrow()[i].draw(f, area);
            }
        }
    }
}

impl<'a, B: Backend> StatefulComponent<B> for SearchBar<'a> {}
impl<'a, B: Backend> StatefulComponent<B> for Level<'a> {}
impl<'a, B: Backend, T: 'static + Clone> StatefulComponent<B> for SearchResults<T>
where T: Component<B>, ListItem<'a>: From<T> {}
impl<'a, T, B: Backend> StatefulComponent<B> for StatefulList<'a, T>
where
    T: Into<Cow<'static, str>> + Clone + Into<String> {}
