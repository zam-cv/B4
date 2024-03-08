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
    crop_sections (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 50]
        crop_type_id -> Nullable<Varchar>,
        units -> Integer,
    }
}

diesel::table! {
    crop_types (name) {
        #[max_length = 50]
        name -> Varchar,
        price -> Integer,
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
        max_sections -> Integer,
        #[max_length = 30]
        ip -> Nullable<Varchar>,
        #[max_length = 50]
        os -> Nullable<Varchar>,
    }
}

diesel::joinable!(crop_sections -> crop_types (crop_type_id));
diesel::joinable!(crop_sections -> users (user_id));
diesel::joinable!(statistics -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    crop_sections,
    crop_types,
    statistics,
    users,
);
