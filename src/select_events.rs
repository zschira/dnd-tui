pub enum SelectResponse {
    Number{name: String, value: i32},
    Filter{name: String, value: String},
    Search{name: String, value: String},
    None,
}
