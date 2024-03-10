// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 20]
        user_type -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        age -> Integer,
        #[max_length = 15]
        gender -> Varchar,
        #[max_length = 30]
        ip -> Nullable<Varchar>,
        #[max_length = 50]
        os -> Nullable<Varchar>,
        player_id -> Integer,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        current_cycle -> Integer,
        current_score -> Integer,
        current_balance -> Integer,
        max_plots -> Integer,
    }
}

diesel::table! {
    loans (id) {
        id -> Integer,
        #[max_length = 10]
        status -> Varchar,
        #[max_length = 10]
        cycle -> Integer,
        amount -> Integer,
        #[max_length = 10]
        creditor -> Varchar,
        player_id -> Integer,
    }
}

diesel::table! {
    insurance (id) {
        id -> Integer,
        #[max_length = 10]
        insurance_type -> Varchar,
        sum_assured -> Integer,
        loan_id -> Integer,
    }
}

diesel::table! {
    statistics (id) {
        id -> Integer,
        cycle -> Integer,
        score -> Integer,
        player_id -> Integer,
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
    plots (id) {
        id -> Integer,
        #[max_length = 50]
        crop_type_id -> Nullable<Varchar>,
        player_id -> Integer,
    }
}

diesel::joinable!(users -> players (player_id));
diesel::joinable!(insurance -> loans (loan_id));
diesel::joinable!(loans -> players (player_id));
diesel::joinable!(statistics -> players (player_id));
diesel::joinable!(plots -> crop_types (crop_type_id));
diesel::joinable!(plots -> players (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    users,
    players,
    loans,
    insurance,
    statistics,
    crop_types,
    plots,
);