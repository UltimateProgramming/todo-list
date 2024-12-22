// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        description -> Text,
        is_done -> Bool,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}
