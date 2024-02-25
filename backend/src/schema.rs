// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    users,
);
