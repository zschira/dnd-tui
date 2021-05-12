use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::convert::TryFrom;
use log::info;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::models::{Class, School};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use super::schema::spells;

#[derive(Insertable)]
#[table_name="spells"]
struct NewSpell<'a> {
    id: i32,
    name: &'a str,
    description: &'a str,
    higher_level: Option<&'a str>,
    range: Option<&'a str>,
    verbal: bool,
    somatic: bool,
    material: bool,
    material_text: Option<&'a str>,
    ritual: bool,
    duration: &'a str,
    concentration: bool,
    casting_time: &'a str,
    level: i32,
    school: &'a str,
    classes: &'a str,
    subclasses: &'a str,
}

#[derive(Queryable, Clone)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub higher_level: Option<String>,
    pub range: Option<String>,
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub material_text: Option<String>,
    pub ritual: bool,
    pub duration: String,
    pub concentration: bool,
    pub casting_time: String,
    pub level: i32,
    pub school: String,
    pub classes: String,
    pub subclasses: String,
}

fn insert_spell(i: usize, spell: &Value, conn: &SqliteConnection) {
    info!("{}", spell["name"].as_str().unwrap());
    diesel::insert_into(spells::table)
        .values(& NewSpell {
            id: i as i32,
            name: spell["name"].as_str().unwrap(),
            description: spell["desc"].as_array()
                .unwrap()
                .iter()
                .fold(String::new(), |acc, sentence| acc + sentence.as_str().unwrap())
                .as_str(),
            higher_level: Some(match spell["higher_level"] {
                Value::Null => String::from(""),
                _ => String::from(spell["higher_level"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, sentence| acc + sentence.as_str().unwrap())
                    .as_str().clone())
            }.as_str()),
            range: match spell["range"] {
                Value::Null => None,
                _ => Some(spell["range"].as_str().unwrap()),
            },
            verbal: spell["components"].as_array()
                .unwrap()
                .iter()
                .any(|comp| comp.as_str().unwrap() == "V"),
            somatic: spell["components"].as_array()
                .unwrap()
                .iter()
                .any(|comp| comp.as_str().unwrap() == "S"),
            material: spell["components"].as_array()
                .unwrap()
                .iter()
                .any(|comp| comp.as_str().unwrap() == "M"),
            material_text: match spell["material"] {
                Value::Null => None,
                _ => Some(spell["material"].as_str().unwrap()),
            },
            ritual: spell["ritual"].as_bool().unwrap(),
            duration: spell["duration"].as_str().unwrap(),
            concentration: spell["concentration"].as_bool().unwrap(),
            casting_time: spell["casting_time"].as_str().unwrap(),
            level: spell["level"].as_i64().unwrap() as i32,
            school: String::from(spell["school"]
                .as_object()
                .unwrap()["name"]
                .as_str()
                .unwrap()).to_lowercase().as_str(),
            classes: spell["classes"]
                .as_array()
                .unwrap()
                .iter()
                .fold(String::new(), |acc, class| {
                    acc + "," + class.as_object().unwrap()["name"].as_str().unwrap()
                }).to_lowercase().as_str(),
            subclasses: "",
        }
    )
    .execute(conn)
    .expect("FAILED");
}

#[derive(Default)]
pub struct Query {
    pub class: Option<Class>,
    pub school: Option<School>,
    pub level: Option<i32>,
}

pub fn query_spell(spell_query: &Query, conn: &SqliteConnection) -> Vec<Spell> {
    let mut query = spells::table.into_boxed();

    if let Some(class) =  spell_query.class {
        query = query.filter(
            spells::classes.like(format!("%{}%", Into::<String>::into(class)))
        );
    }

    if let Some(school) = spell_query.school {
        query = query.filter(
            spells::school.eq(Into::<String>::into(school))
        );
    }

    if let Some(level) = spell_query.level {
        if level > 0 {
            query = query.filter(
                spells::level.eq(level as i32)
            );
        }
    }

    query.load(conn)
        .expect("Failed to query for spell")
}

pub fn build_db(conn: &SqliteConnection) {
    let json_path = env::var("SPELLS_JSON")
        .expect("Spells json path must be set");

    let file = File::open(Path::new(&json_path)).expect("Failed to open json file");
    let reader = BufReader::new(file);
    let spells: Vec<Value> = serde_json::from_reader(reader)
        .expect("Failed to read json");
    spells.iter().enumerate()
        .for_each(|(i, spell)| insert_spell(i, spell, conn));
}
