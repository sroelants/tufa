// @generated automatically by Diesel CLI.

diesel::table! {
    services (id) {
        id -> Nullable<Integer>,
        name -> Text,
        secret -> Text,
        encrypted -> Integer,
        created_at -> Integer,
        last_used_at -> Nullable<Integer>,
    }
}
