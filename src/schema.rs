table! {
    spells (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        higher_level -> Nullable<Text>,
        range -> Nullable<Text>,
        verbal -> Bool,
        somatic -> Bool,
        material -> Bool,
        material_text -> Nullable<Text>,
        ritual -> Bool,
        duration -> Text,
        concentration -> Bool,
        casting_time -> Text,
        level -> Integer,
        school -> Text,
        classes -> Text,
        subclasses -> Text,
    }
}
