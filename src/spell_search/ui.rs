
use crate::app::{Component, StatefulComponent};
use crate::components::Container;
use crate::documents::Spell;

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, ListItem, Paragraph, Wrap};
use tui::text::{Span, Spans};
use tui::style::{Modifier, Style};
use tui::Frame;
use tui::backend::Backend;
use std::borrow::Cow;

#[derive(Default)]
pub struct SpellSearchUi;

impl<B: Backend> Component<B> for Container<SpellSearchUi, B> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        self.children[0].draw(f, chunks[0]);
        self.children[1].draw(f, chunks[1]);
    }
}

#[derive(Default)]
pub struct SearchMain;

impl<B: Backend> Component<B> for Container<SearchMain, B> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .direction(Direction::Horizontal)
            .split(area);

        self.children[0].draw(f, chunks[0]);
        self.children[1].draw(f, chunks[1]);
    }
}

#[derive(Default)]
pub struct Filters;

impl<B: Backend> Component<B> for Container<Filters, B> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(45),
                Constraint::Percentage(45),
                Constraint::Percentage(10)
            ].as_ref())
            .split(area);

        self.children[0].draw(f, chunks[0]);
        self.children[1].draw(f, chunks[1]);
        self.children[2].draw(f, chunks[2]);
    }
}

impl<B: Backend> Component<B> for Spell {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        let paragraph = Paragraph::new(vec![
            Spans::from(vec![
                Span::styled(
                    self.name.clone(),
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                Span::from("\n"),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("{}", self.level),
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::styled(
                    match self.level {
                        1 => "st",
                        2 => "nd",
                        3 => "rd",
                        _ => "th",
                    },
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::styled(
                    " level ",
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::styled(
                    self.school.name.clone(),
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::from("\n"),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Casting time: ",
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                Span::from(self.casting_time.clone()),
            ]),
            if let Some(range) = &self.range {
                Spans::from(vec![
                    Span::styled(
                        "Range: ",
                        Style::default().add_modifier(Modifier::BOLD)
                    ),
                    Span::from(range.clone()),
                ])
            } else { Spans::from(vec![]) },
            Spans::from(vec![
                Span::styled(
                    "Materials: ",
                    Style::default().add_modifier(Modifier::BOLD)
                )
            ]),
            Spans::from(
                self.components.iter()
                    .map(|component| Span::from(component.as_str()))
                    .collect::<Vec<Span>>()
            ),
            Spans::from(vec![
                if let Some(material) = self.material.clone() {
                    Span::from("(".to_owned() + &material + ")")
                } else { Span::from("") },
            ]),
            Spans::from(vec![
                Span::from("\n"),
                Span::from(
                    self.desc.iter()
                        .fold(String::new(), |acc, sentence| {
                            acc + sentence
                        })
                ),
                Span::from("\n"),
            ]),
            if let Some(higher_level) = self.higher_level.clone() {
                Spans::from(vec![
                    Span::styled(
                        "At higher levels: ",
                        Style::default().add_modifier(Modifier::BOLD)
                    ),
                    Span::from(
                        higher_level.iter()
                            .fold(String::new(), |acc, sentence| {
                                acc + sentence
                            })
                    ),
                ])
            } else { Spans::from(vec![]) },
        ])
            .block(
                Block::default()
                .title("Search Results")
                .borders(Borders::ALL)
            )
            .wrap(Wrap{ trim: false })
            .alignment(Alignment::Left);
        f.render_widget(paragraph, area);
    }
}

impl<'a> From<Spell> for ListItem<'a> {
    fn from(spell: Spell) -> ListItem<'a> {
        ListItem::new(vec![Spans::from(Span::raw(spell.name.clone()))])
    }
}

impl<B: Backend> StatefulComponent<B> for Container<Filters, B> {}
impl<B: Backend> StatefulComponent<B> for Container<SpellSearchUi, B> {}
impl<B: Backend> StatefulComponent<B> for Container<SearchMain, B> {}
