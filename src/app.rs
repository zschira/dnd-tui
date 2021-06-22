use tui::layout::{Rect, Direction};
use tui::backend::Backend;
use tui::Frame;
use termion::event::Key;
use std::rc::Rc;

use crate::db_utils::{build_db, establish_connection};
use crate::spell_search::{build_component_tree, SpellSearch};
use crate::select_events::SelectResponse;
use crate::input_buffer::InputBuffer;
use ejdb::Database;
use log::info;

pub enum MoveResponse {
    Sibling,
    None,
}

pub enum AppMode {
    Search,
    Navigation,
}

pub trait Stateful {
    fn next(&mut self, direction: Direction) -> MoveResponse;
    fn previous(&mut self, direction: Direction) -> MoveResponse;
    fn hover(&mut self, activate: bool);
    fn select(&mut self, activate: bool) -> SelectResponse;
}

pub trait Component<B: Backend> {
    fn draw(&self, f: &mut Frame<B>, area: Rect);
}

pub trait StatefulComponent<B: Backend>: Stateful + Component<B>
{}

pub struct App<'a, B: Backend> {
    pub title: &'a str,
    pub component_tree: SpellSearch<B>,
    pub should_quit: bool,
    pub db: Rc<Database>,
    pub search_buf: InputBuffer,
    pub app_mode: AppMode,
}

impl<'a, B: 'static + Backend> App<'a, B> {
    pub fn new(title: &'a str) -> App<'a, B> {
        let db = Rc::new(establish_connection());
        let component_tree = build_component_tree(Rc::clone(&db));
        let mut app = App {
            title,
            component_tree,
            should_quit: false,
            db,
            search_buf: InputBuffer::new(),
            app_mode: AppMode::Navigation,
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

    pub fn on_select(&mut self) {
        self.component_tree.select(true);
    }

    pub fn on_key(&mut self, key: Key) {
        match self.app_mode {
            AppMode::Navigation => {
                match key {
                    Key::Up | Key::Char('k') => { self.on_up() },
                    Key::Down | Key::Char('j') => { self.on_down() },
                    Key::Left | Key::Char('h') => { self.on_left() },
                    Key::Right | Key::Char('l') => { self.on_right() },
                    Key::Char('\n') => { self.on_select() },
                    Key::Esc => { self.on_unselect() },
                    Key::Char('q') => { self.should_quit = true; },
                    Key::Char('/') => { self.app_mode = AppMode::Search; },
                    _ => {},
                }
            },
            AppMode::Search => {
                let exit = self.component_tree.key_stroke(key);
                if exit { self.app_mode = AppMode::Navigation; }
            },
        }
    }
}
