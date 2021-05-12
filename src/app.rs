use diesel::sqlite::SqliteConnection;
use tui::layout::{Rect, Direction};
use tui::backend::Backend;
use tui::Frame;

use crate::db_utils::{build_db, establish_connection, query_spell, Query, Spell};
use crate::components::Container;
use crate::components_ui::{build_component_tree, SpellSearch};
use crate::models::{Class, School};
use log::info;

pub enum MoveResponse {
    Sibling,
    None,
}

pub enum SelectResponse {
    Level(i32),
    Class(Class),
    School(School),
    None,
}

pub trait Stateful {
    fn next(&mut self, direction: Direction) -> MoveResponse;
    fn previous(&mut self, direction: Direction) -> MoveResponse;
    fn hover(&mut self, activate: bool);
    fn select(&mut self, activate: bool) -> SelectResponse;
}

pub trait Component<B: Backend> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, spells: &Option<Vec<Spell>>);
}

pub trait StatefulComponent<B: Backend>: Stateful + Component<B>
{}

pub struct App<'a, B: Backend> {
    pub title: &'a str,
    pub component_tree: Container<SpellSearch, B>,
    pub search_results: Option<Vec<Spell>>,
    pub should_quit: bool,
    pub spell_query: Query,
    pub conn: SqliteConnection
}

impl<'a, B: 'static + Backend> App<'a, B> {
    pub fn new(title: &'a str) -> App<'a, B> {
        let mut app = App {
            title,
            search_results: None,
            component_tree: build_component_tree(),
            should_quit: false,
            spell_query: Query::default(),
            conn: establish_connection()
        };
        app.component_tree.hover(true);
        app
    }

    pub fn on_up(&mut self) {
        self.component_tree.previous(Direction::Vertical);
    }

    pub fn on_down(&mut self) {
        self.component_tree.next(Direction::Vertical);
    }

    pub fn on_left(&mut self) {
        self.component_tree.previous(Direction::Horizontal);
    }

    pub fn on_right(&mut self) {
        self.component_tree.next(Direction::Horizontal);
    }

    pub fn on_unselect(&mut self) {
        self.component_tree.select(false);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            '\n' => {
                match self.component_tree.select(true) {
                    SelectResponse::Class(class) => {
                        self.spell_query.class = Some(class);
                        self.search_results = 
                            Some(query_spell(&self.spell_query, &self.conn));
                    },
                    SelectResponse::Level(level) => {
                        self.spell_query.level = Some(level);
                        self.search_results = 
                            Some(query_spell(&self.spell_query, &self.conn));
                    },
                    SelectResponse::School(school) => {
                        self.spell_query.school = Some(school);
                        self.search_results = 
                            Some(query_spell(&self.spell_query, &self.conn));
                    },
                    _ => {}
                }
            },
            'x' => {
                self.spell_query = Query::default();
                self.search_results = 
                    Some(query_spell(&self.spell_query, &self.conn));
            },
            'r' => build_db(&self.conn),
            _ => {}
        }
    }
}
