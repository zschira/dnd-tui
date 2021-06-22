use ejdb::Database;
use ejdb::query::{Q, QH};
use std::rc::Rc;

pub fn get_names(collection: &str, db: &Rc<Database>) -> Vec<String> {
    db.collection(collection).unwrap()
        .query(Q.empty(), QH.field("name").include())
        .find()
        .unwrap()
        .map(|name| name.unwrap().get_str("name").unwrap().to_string())
        .collect()
}
