use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::convert::TryFrom;

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
    higher_level: &'a str,
    verbal: bool,
    somatic: bool,
    material: bool,
    material_text: &'a str,
    ritual: bool,
    duration: &'a str,
    concentration: bool,
    casting_time: &'a str,
    level: i32,
    school: i32,
    classes: &'a [u8],
    subclasses: &'a [u8],
}

fn insert_spell(i: usize, spell: &Value, conn: &SqliteConnection) {
    diesel::insert_into(spells::table)
        .values(& NewSpell {
            id: i as i32,
            name: spell["name"].as_str().unwrap(),
            description: spell["desc"].as_array()
                .unwrap()
                .iter()
                .fold(String::new(), |acc, sentence| acc + sentence.as_str().unwrap())
                .as_str(),
            higher_level: match spell["higher_level"] {
                Value::Null => String::from(""),
                _ => String::from(spell["higher_level"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, sentence| acc + sentence.as_str().unwrap())
                    .as_str())
            }.as_str(),
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
                Value::Null => "",
                _ => spell["material"].as_str().unwrap(),
            },
            ritual: spell["ritual"].as_bool().unwrap(),
            duration: spell["duration"].as_str().unwrap(),
            concentration: spell["concentration"].as_bool().unwrap(),
            casting_time: spell["casting_time"].as_str().unwrap(),
            level: spell["level"].as_i64().unwrap() as i32,
            school: School::try_from(String::from(spell["school"]
                .as_object()
                .unwrap()["name"]
                .as_str()
                .unwrap()).to_lowercase()).unwrap() as i32,
            classes: &(spell["classes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|class| Class::try_from(
                    String::from(class.as_object().unwrap()["name"].as_str().unwrap())
                    .to_lowercase()
                ).unwrap() as u8)
                .collect::<Vec<u8>>()[..]),
            subclasses: &[0],
        }
    )
    .execute(conn)
    .expect("FAILED");

    /*
    diesel::insert_into(spells::table)
        .values(&spell)
        .execute(conn)
        .expect("Failed to insert spell");
        */
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
