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
