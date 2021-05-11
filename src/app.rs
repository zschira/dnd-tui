use diesel::sqlite::SqliteConnection;
use tui::layout::{Rect, Direction};
use tui::backend::Backend;
use tui::Frame;

use crate::db_utils::establish_connection;

pub enum MoveResponse {
    Sibling,
    None,
}

pub trait Stateful {
    fn next(&mut self, direction: Direction) -> MoveResponse;
    fn previous(&mut self, direction: Direction) -> MoveResponse;
    fn hover(&mut self, activate: bool);
    fn select(&mut self, activate: bool);
}

pub trait Component {
    fn draw<B: Backend>(&self, f: Frame<B>, area: Rect);
}

pub trait StatefulComponent: Stateful + Component {}
