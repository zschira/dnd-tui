use crate::app::{MoveResponse, Stateful, StatefulComponent};
use crate::input_buffer::InputBuffer;
use crate::select_events::SelectResponse;
use tui::layout::Direction;
use tui::widgets::ListState;
use tui::backend::Backend;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Container<C: Default, B: Backend> {
    pub children: Vec<Box<dyn StatefulComponent<B>>>,
    pub direction: Direction,
    pub selected: usize,
    _container: C
}

impl<C: Default, B: Backend> Container<C, B> {
    pub fn with_items(items: Vec<Box<dyn StatefulComponent<B>>>, direction: Direction)
    -> Container<C, B> {
        Container {
            children: items,
            direction,
            selected: 0,
            _container: C::default()
        }
    }
}

impl<C: Default, B: Backend> Stateful for Container<C, B> {
    fn next(&mut self, direction: Direction) -> MoveResponse {
        if self.direction != direction {
            return self.children[self.selected].next(direction);
        }

        match self.children[self.selected].next(direction) {
            MoveResponse::Sibling => {
                if self.selected < self.children.len() - 1 {
                    self.children[self.selected].hover(false);
                    self.selected += 1;
                    self.children[self.selected].hover(true);
                    MoveResponse::None
                } else {
                    MoveResponse::Sibling
                }
            },
            MoveResponse::None => { MoveResponse::None }
        }
    }

    fn previous(&mut self, direction: Direction) -> MoveResponse {
        if self.direction != direction {
            return self.children[self.selected].previous(direction);
        }

        match self.children[self.selected].previous(direction) {
            MoveResponse::Sibling => {
                if self.selected > 0 {
                    self.children[self.selected].hover(false);
                    self.selected -= 1;
                    self.children[self.selected].hover(true);
                    MoveResponse::None
                } else {
                    MoveResponse::Sibling
                }
            },
            MoveResponse::None => { MoveResponse::None }
        }
    }

    fn hover(&mut self, activate: bool) { self.children[self.selected].hover(activate) }
    fn select(&mut self, activate: bool) -> SelectResponse {
        self.children[self.selected].select(activate)
    }
}

pub enum SelectState {
    Highlighted,
    Selected,
    None
}

pub struct StatefulList<'a, T>
where
    T: Into<Cow<'static, str>>
{
    pub state: ListState,
    pub items: Vec<T>,
    pub name: &'a str,
    pub selected: SelectState,
}

impl<'a, T> StatefulList<'a, T>
where
    T: Into<Cow<'static, str>> + Into<String>
{
    pub fn with_items(items: Vec<T>, name: &'a str) -> StatefulList<'a, T> {
        StatefulList {
            state: ListState::default(),
            items,
            name,
            selected: SelectState::None,
        }
    }
}

impl<'a, T> Stateful for StatefulList<'a, T>
where
    T: Into<Cow<'static, str>> + Into<String> + Clone
{
    fn next(&mut self, direction: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                match direction {
                    Direction::Horizontal => MoveResponse::None,
                    Direction::Vertical => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i >= self.items.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            },
                            None => 0,
                        };
                        self.state.select(Some(i));
                        MoveResponse::None
                    }
                }
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn previous(&mut self, direction: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                match direction {
                    Direction::Horizontal => MoveResponse::None,
                    Direction::Vertical => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i <= 0 {
                                    self.items.len() - 1
                                } else {
                                    i - 1
                                }
                            },
                            None => 0,
                        };
                        self.state.select(Some(i));
                        MoveResponse::None
                    }
                }
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn hover(&mut self, activate: bool) {
        if activate {
            self.selected = SelectState::Highlighted;
        } else {
            self.selected = SelectState::None;
        }
    }

    fn select(&mut self, activate: bool) -> SelectResponse {
        if activate {
            match self.selected {
                SelectState::Selected => {
                    self.selected = SelectState::Highlighted;
                    if let Some(i) = self.state.selected() {
                        SelectResponse::Filter{
                            name: self.name.to_string(),
                            value: Into::<String>::into(self.items[i].clone()).to_lowercase()
                        }
                    } else {
                        SelectResponse::None
                    }
                },
                _ => {
                    self.selected = SelectState::Selected;
                    SelectResponse::None
                },
            }
        } else {
            self.selected = SelectState::None;
            SelectResponse::None
        }
    }
}

pub struct SearchResults< T> {
    pub state: ListState,
    pub items: Rc<RefCell<Vec<T>>>,
    pub selected: SelectState,
    pub item_view: bool,
}

impl<T> SearchResults<T> {
    pub fn new(items: Rc<RefCell<Vec<T>>>) -> SearchResults<T> {
        SearchResults {
            state: ListState::default(),
            items,
            selected: SelectState::None,
            item_view: false,
        }
    }
}

impl<T> Stateful for SearchResults<T> {
    fn next(&mut self, direction: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                match direction {
                    Direction::Horizontal => MoveResponse::None,
                    Direction::Vertical => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i >= self.items.borrow().len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            },
                            None => 0,
                        };
                        self.state.select(Some(i));
                        MoveResponse::None
                    }
                }
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn previous(&mut self, direction: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                match direction {
                    Direction::Horizontal => MoveResponse::None,
                    Direction::Vertical => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i <= 0 {
                                    self.items.borrow().len() - 1
                                } else {
                                    i - 1
                                }
                            },
                            None => 0,
                        };
                        self.state.select(Some(i));
                        MoveResponse::None
                    }
                }
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn hover(&mut self, activate: bool) {
        if activate {
            self.selected = SelectState::Highlighted;
        } else {
            self.selected = SelectState::None;
        }
    }

    fn select(&mut self, activate: bool) -> SelectResponse {
        if activate {
            match self.selected {
                SelectState::Selected => {
                    self.item_view = true;
                },
                _ => {
                    self.selected = SelectState::Selected;
                },
            }
        } else {
            self.item_view = false;
            self.selected = SelectState::None;
        }

        SelectResponse::None
    }
}

pub struct SearchBar<'a> {
    pub name: &'a str,
    pub value: Rc<RefCell<InputBuffer>>,
    pub selected: SelectState,
}

impl<'a> SearchBar<'a> {
    pub fn new(name: &'a str, value: Rc<RefCell<InputBuffer>>) -> SearchBar<'a> {
        SearchBar {
            name,
            value,
            selected: SelectState::None,
        }
    }
}

impl<'a> Stateful for SearchBar<'a> {
    fn next(&mut self, _: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => MoveResponse::None,
            _ => MoveResponse::Sibling,
        }
    }

    fn previous(&mut self, _: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => MoveResponse::None,
            _ => MoveResponse::Sibling,
        }
    }

    fn hover(&mut self, activate: bool) {
        if activate {
            self.selected = SelectState::Highlighted;
        } else {
            self.selected = SelectState::None;
        }
    }

    fn select(&mut self, activate: bool) -> SelectResponse {
        if activate {
            self.selected = SelectState::Selected;
        } else {
            self.selected = SelectState::None;
        }
        SelectResponse::None
    }
}

pub struct Level<'a> {
    pub name: &'a str,
    pub level: i32,
    pub selected: SelectState,
}

impl<'a> Level<'a> {
    pub fn new(name: &'a str) -> Level<'a> {
        Level {
            name,
            level: 0,
            selected: SelectState::None
        }
    }
}

impl<'a> Stateful for Level<'a> {
    fn next(&mut self, _: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                if self.level < 20 { self.level += 1; }
                MoveResponse::None
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn previous(&mut self, _: Direction) -> MoveResponse {
        match self.selected {
            SelectState::Selected => {
                if self.level > 0 { self.level -= 1; }
                MoveResponse::None
            },
            _ => MoveResponse::Sibling,
        }
    }

    fn hover(&mut self, activate: bool) {
        if activate {
            self.selected = SelectState::Highlighted;
        } else {
            self.selected = SelectState::None;
        }
    }

    fn select(&mut self, activate: bool) -> SelectResponse {
        if activate {
            match self.selected {
                SelectState::Selected => {
                    self.selected = SelectState::None;
                    SelectResponse::Number {
                        name: self.name.to_string(),
                        value: self.level
                    }
                },
                _ => {
                    self.selected = SelectState::Selected;
                    SelectResponse::None
                }
            }
        } else {
            self.selected = SelectState::None;
            SelectResponse::None
        }
    }
}
