use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::convert::TryFrom;
use ejdb::Database;
use ejdb::bson;

use crate::documents::*;

pub fn establish_connection() -> Database {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    Database::open(database_url).unwrap()
}


#[macro_use]
mod macs {
    macro_rules! deserialize_json {
        ($db:expr, $json_path:expr, $( ($tp:ty, $doc:expr)), * ) => {
            $(
                println!("Adding {}", $doc);
                let reader = get_json(&$json_path, $doc);
                let collection = $db.collection($doc).unwrap();

                let json: Vec<$tp> = serde_json::from_reader(reader)
                .expect("Failed to read json");

                json.iter()
                    .for_each(|json_doc| {
                        let doc = bson::to_bson(&json_doc)
                            .unwrap()
                            .as_document()
                            .unwrap()
                            .clone();
                        collection.save(doc).unwrap();
                    });
            )*
        }
    }
}

pub fn build_db(db: &Database) {
    let json_path = env::var("JSON")
        .expect("Spells json path must be set");

    deserialize_json!(
        db,
        json_path,
        (AbilityScore, "Ability-Scores"),
        (Alignment, "Alignments"),
        (Background, "Backgrounds"),
        (Class, "Classes"),
        (Condition, "Conditions"),
        (DamageType, "Damage-Types"),
        (Equipment, "Equipment"),
        (EquipmentCategory, "Equipment-Categories"),
        (Feature, "Features"),
        (Language, "Languages"),
        (Level, "Levels"),
        (MagicItem, "Magic-Items"),
        (Monster, "Monsters"),
        (Proficiency, "Proficiencies"),
        (Race, "Races"),
        (RuleSection, "Rule-Sections"),
        (Rule, "Rules"),
        (School, "Magic-Schools"),
        (Skill, "Skills"),
        (Spell, "Spells"),
        (StartingEquipment, "StartingEquipment"),
        (Subclass, "Subclasses"),
        (Subrace, "Subraces"),
        (Trait, "Traits"),
        (WeaponProperty, "Weapon-Properties")
    );
}

fn get_json(prefix: &String, suffix: &str) -> BufReader<File> {
    let file = File::open(Path::new(&(prefix.to_owned() + suffix + ".json")))
        .expect("Failed to open json file");
    BufReader::new(file)
}
