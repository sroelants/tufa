// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Integer,
        salt -> Text,
        digest -> Text,
        created_at -> Integer,
    }
}

diesel::table! {
    services (id) {
        id -> Integer,
        name -> Text,
        secret_digest -> Text,
        password -> Nullable<Integer>,
        created_at -> Integer,
        last_used_at -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    passwords,
    services,
);
