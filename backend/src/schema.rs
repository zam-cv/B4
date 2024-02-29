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
    statistics (id) {
        user_id -> Integer,
        id -> Integer,
        date -> Timestamp,
        punctuation -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        balance_cash -> Integer,
        balance_verqor -> Integer,
        balance_coyote -> Integer,
        current_day -> Timestamp,
    }
}

diesel::joinable!(statistics -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    statistics,
    users,
);
