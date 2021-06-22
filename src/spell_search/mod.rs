use ejdb::Database;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Direction, Rect};
use ejdb::query::{Q, QH, Query};
use ejdb::bson;
use std::rc::Rc;
use std::cell::RefCell;
use termion::event::Key;
use crate::documents::Spell;
use crate::app::{Component, MoveResponse, Stateful};
use crate::components::{Container, Level, SearchBar, SearchResults, StatefulList};
use crate::select_events::SelectResponse;
use crate::input_buffer::InputBuffer;
use log::info;

mod ui;
mod util;

pub use crate::spell_search::ui::*;
use crate::spell_search::util::get_names;

pub struct SpellSearch<B: Backend> {
    pub search_results: Rc<RefCell<Vec<Spell>>>,
    pub input_buff: Rc<RefCell<InputBuffer>>,
    query_items: SpellQuery,
    ui_components: Option<Container<SpellSearchUi, B>>,
    db: Rc<Database>
}

impl<B: Backend> Component<B> for SpellSearch<B> {
    fn draw(&self, f: &mut Frame<B>, area: Rect) {
        if let Some(ui_components) = &self.ui_components {
            ui_components.draw(f, area);
        }
    }
}

impl<B: Backend> SpellSearch<B> {
    pub fn new(db: Rc<Database>) -> SpellSearch<B> {
        SpellSearch {
            search_results: Rc::new(RefCell::new(Vec::new())),
            input_buff: Rc::new(RefCell::new(InputBuffer::new())),
            query_items: SpellQuery::default(),
            ui_components: None,
            db
        }
    }

    pub fn key_stroke(&mut self, key: Key) -> bool {
        let response = self.input_buff.borrow_mut().key_stroke(key);
        if let SelectResponse::Search{name, value} = response {
            self.query_items.term = Some(value);
            let spells = query_spell(&self.query_items, &self.db);
            *self.search_results.borrow_mut().as_mut() = spells;
            true
        } else {
            false
        }
    }
}

impl< B: Backend> Stateful for SpellSearch< B> {
    fn hover(&mut self, activate: bool) {
        if let Some(ui_components) = &mut self.ui_components {
            ui_components.hover(activate);
        }
    }

    fn next(&mut self, direction: Direction) -> MoveResponse {
        if let Some(ui_components) = &mut self.ui_components {
            ui_components.next(direction)
        } else {
            MoveResponse::None
        }
    }

    fn previous(&mut self, direction: Direction) -> MoveResponse {
        if let Some(ui_components) = &mut self.ui_components {
            ui_components.previous(direction)
        } else {
            MoveResponse::None
        }
    }

    fn select(&mut self, activate: bool) -> SelectResponse {
        if let Some(ui_components) = &mut self.ui_components {
            match ui_components.select(activate) {
                SelectResponse::Number{value, ..} => {
                    self.query_items.level = Some(value);
                    let spells = query_spell(&self.query_items, &self.db);
                    *self.search_results.borrow_mut().as_mut() = spells;
                },
                SelectResponse::Filter{name, value}  => {
                    match name.as_str() {
                        "Class" => { self.query_items.class = Some(value); },
                        "School" => { self.query_items.school = Some(value); },
                        _ => {}
                    }
                    let spells = query_spell(&self.query_items, &self.db);
                    *self.search_results.borrow_mut().as_mut() = spells;
                },
                SelectResponse::Search{value, ..} => {
                    self.query_items.term = Some(value);
                    let spells = query_spell(&self.query_items, &self.db);
                    *self.search_results.borrow_mut().as_mut() = spells;
                },
                _ => {}
            }
        }

        SelectResponse::None
    }
}

#[derive(Default)]
pub struct SpellQuery {
    pub class: Option<String>,
    pub school: Option<String>,
    pub level: Option<i32>,
    pub term: Option<String>
}

pub fn query_spell(spell_query: &SpellQuery, db: &Database) -> Vec<Spell> {
    let mut query = Query::new();

    if let Some(class) = &spell_query.class {
        query = query.field("classes")
            .elem_match(
                Q.field("index")
                    .eq(class)
            );
    }

    if let Some(school) = &spell_query.school {
        query = query.field("school.index")
            .eq(school);
    }

    if let Some(level) = spell_query.level {
        if level > 0 {
            query = query.field("level")
                .eq(level);
        }
    }

    if let Some(term) = &spell_query.term {
        query = query.field("name")
            .case_insensitive()
            .eq(term)
    }

    let spell_coll = db.collection("Spells").unwrap();
    spell_coll.query(query, QH.empty())
        .find()
        .unwrap()
        .map(|spell| bson::from_bson(
                bson::Bson::Document(spell.unwrap())
            ).unwrap()
        )
        .collect()
}

pub fn build_component_tree<'a, B: 'static + Backend>(db: Rc<Database>)
    -> SpellSearch<B> {
    let mut search = SpellSearch::new(db.clone()); 
    search.ui_components = Some(Container::with_items(
        vec![
            Box::new(SearchBar::new("Search", search.input_buff.clone())),
            Box::new(Container::<SearchMain, B>::with_items(
                vec![
                    Box::new(Container::<Filters, B>::with_items(
                        vec![
                            Box::new(StatefulList::with_items(
                                get_names("Classes", &db),
                                "Class"
                            )),
                            Box::new(StatefulList::with_items(
                                get_names("Magic-Schools", &db),
                                "School"
                            )),
                            Box::new(
                                Level::new("Level")
                            )
                        ],
                        Direction::Vertical
                    )),
                    Box::new(SearchResults::new(search.search_results.clone())),
                ],
                Direction::Horizontal
            ))
        ],
        Direction::Vertical
    ));
    search
}
