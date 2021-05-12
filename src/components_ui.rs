use crate::app::{Component, StatefulComponent};
use crate::components::{Container, Level, SearchBar,
                        SearchResults, SelectState, StatefulList};
use crate::db_utils::Spell;

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use std::borrow::Cow;

#[derive(Default)]
pub struct SpellSearch;

impl<B: Backend> Component<B> for Container<SpellSearch, B> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, spells: &Option<Vec<Spell>>) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        self.children[0].draw(f, chunks[0], &None);
        self.children[1].draw(f, chunks[1], spells);
    }
}

#[derive(Default)]
pub struct SearchMain;

impl<B: Backend> Component<B> for Container<SearchMain, B> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, spells: &Option<Vec<Spell>>) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .direction(Direction::Horizontal)
            .split(area);

        self.children[0].draw(f, chunks[0], &None);
        self.children[1].draw(f, chunks[1], spells);
    }
}

#[derive(Default)]
pub struct Filters;

impl<B: Backend> Component<B> for Container<Filters, B> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _: &Option<Vec<Spell>>) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(45),
                Constraint::Percentage(45),
                Constraint::Percentage(10)
            ].as_ref())
            .split(area);

        self.children[0].draw(f, chunks[0], &None);
        self.children[1].draw(f, chunks[1], &None);
        self.children[2].draw(f, chunks[2], &None);
    }
}

impl<'a, B: Backend> Component<B> for SearchBar<'a> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _: &Option<Vec<Spell>>) {
        let paragraph = Paragraph::new(Spans::from(vec![
                Span::raw(self.value.as_str())
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
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _: &Option<Vec<Spell>>) {
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
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _: &Option<Vec<Spell>>) {
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

        f.render_stateful_widget(items, area, &mut self.state);
    }
}

impl SearchResults {
    fn list<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.name.clone()))]))
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

        f.render_stateful_widget(items, area, &mut self.state);
    }

    fn card<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect, i: usize) {
        let paragraph = Paragraph::new(vec![
            Spans::from(vec![
                Span::styled(
                    self.items[i].name.clone(),
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                Span::from("\n"),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!("{}", self.items[i].level),
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::styled(
                    match self.items[i].level {
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
                    self.items[i].school.clone(),
                    Style::default().add_modifier(Modifier::ITALIC)
                ),
                Span::from("\n"),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Casting time: ",
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                Span::from(self.items[i].casting_time.clone()),
            ]),
            if let Some(range) = self.items[i].range.clone() {
                Spans::from(vec![
                    Span::styled(
                        "Range: ",
                        Style::default().add_modifier(Modifier::BOLD)
                    ),
                    Span::from(range),
                ])
            } else { Spans::from(vec![]) },
            Spans::from(vec![
                Span::styled(
                    "Materials: ",
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                if self.items[i].verbal {
                    Span::from("V ")
                } else { Span::from("") },
                if self.items[i].somatic {
                    Span::from("S ")
                } else { Span::from("") },
                if self.items[i].material {
                    Span::from("M ")
                } else { Span::from("") },
                if let Some(material) = self.items[i].material_text.clone() {
                    Span::from("(".to_owned() + &material + ")")
                } else { Span::from("") },
            ]),
            Spans::from(vec![
                Span::from("\n"),
                Span::from(self.items[i].description.clone()),
                Span::from("\n"),
            ]),
            if let Some(higher_level) = self.items[i].higher_level.clone() {
                Spans::from(vec![
                    Span::styled(
                        "At higher levels: ",
                        Style::default().add_modifier(Modifier::BOLD)
                    ),
                    Span::from(higher_level),
                ])
            } else { Spans::from(vec![]) },
        ])
            .block(
                Block::default()
                .title("Search Results")
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
            .wrap(Wrap{ trim: false })
            .alignment(Alignment::Left);
        f.render_widget(paragraph, area);
    }
}

impl<B: Backend> Component<B> for SearchResults {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, spells: &Option<Vec<Spell>>) {
        if let Some(spells) = spells {
            self.items = spells.clone();
        }

        if !self.spell_card {
            self.list(f, area);
        } else {
            if let Some(i) = self.state.selected() {
                self.card(f, area, i);
            }
        }
    }
}

impl<B: Backend> StatefulComponent<B> for Container<SpellSearch, B> {}
impl<B: Backend> StatefulComponent<B> for Container<SearchMain, B> {}
impl<B: Backend> StatefulComponent<B> for Container<Filters, B> {}
impl<'a, B: Backend> StatefulComponent<B> for SearchBar<'a> {}
impl<'a, B: Backend> StatefulComponent<B> for Level<'a> {}
impl<B: Backend> StatefulComponent<B> for SearchResults {}
impl<'a, T, B: Backend> StatefulComponent<B> for StatefulList<'a, T>
where
    T: Into<Cow<'static, str>> + Clone + Into<String> {}

pub fn build_component_tree<B: 'static + Backend>() 
    -> Container<SpellSearch, B> {
    let root: Container<SpellSearch, B> = Container::with_items(
        vec![
            Box::new(SearchBar::new("Search")),
            Box::new(Container::<SearchMain, B>::with_items(
                vec![
                    Box::new(Container::<Filters, B>::with_items(
                        vec![
                            Box::new(StatefulList::with_items(
                                vec![
                                    "Barbarian",
                                    "Bard",
                                    "Cleric",
                                    "Druid",
                                    "Fighter",
                                    "Monk",
                                    "Paladin",
                                    "Ranger",
                                    "Rogue",
                                    "Sorcerer",
                                    "Warlock",
                                    "Wizard"
                                ],
                                "Class"
                            )),
                            Box::new(StatefulList::with_items(
                                vec![
                                    "Abjuration",
                                    "Conjuration",
                                    "Divination",
                                    "Enchantment",
                                    "Evocation",
                                    "Illusion",
                                    "Necromancy",
                                    "Transmutation",
                                ],
                                "School"
                            )),
                            Box::new(
                                Level::new("Level")
                            )
                        ],
                        Direction::Vertical
                    )),
                    Box::new(SearchResults::with_items(
                        vec![],
                    )),
                ],
                Direction::Horizontal
            ))
        ],
        Direction::Vertical
    );
    root
}
