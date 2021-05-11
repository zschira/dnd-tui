use crate::app::{MoveResponse, Stateful};
use tui::layout::Direction;
use tui::widgets::ListState;

pub struct Container<C> {
    children: Vec<Box<dyn Stateful>>,
    direction: Direction,
    selected: usize,
    container: C
}

impl<C> Stateful for Container<C> {
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
            return self.children[self.selected].next(direction);
        }

        match self.children[self.selected].next(direction) {
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
    fn select(&mut self, activate: bool) { self.children[self.selected].select(activate) }
}

pub enum SelectState {
    Highlighted,
    Selected,
    None
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub selected: SelectState,
}

impl<T> Stateful for StatefulList<T> {
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

    fn select(&mut self, activate: bool) {
        if activate {
            self.selected = SelectState::Selected;
        } else {
            self.selected = SelectState::None;
        }
    }
}
