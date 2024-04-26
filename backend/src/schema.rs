// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    admins (id) {
        id -> Integer,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        #[max_length = 150]
        role_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    crop_types (name) {
        #[max_length = 50]
        name -> Varchar,
        price -> Integer,
        duration -> Integer,
        #[max_length = 500]
        description -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    insurance (id) {
        id -> Integer,
        #[max_length = 10]
        insurance_type -> Varchar,
        sum_assured -> Integer,
        loan_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    loans (id) {
        id -> Integer,
        #[max_length = 10]
        status -> Varchar,
        cycle -> Integer,
        amount -> Integer,
        #[max_length = 10]
        creditor -> Varchar,
        player_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    players (id) {
        id -> Integer,
        time_in_game -> Double,
        time -> Integer,
        current_cycle -> Integer,
        current_score -> Double,
        balance_cash -> Integer,
        balance_verqor -> Integer,
        balance_coyote -> Integer,
        max_change -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    plots (id) {
        id -> Integer,
        #[max_length = 50]
        crop_type_id -> Nullable<Varchar>,
        quantity -> Integer,
        growth -> Integer,
        player_id -> Integer
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    statistics (id) {
        id -> Integer,
        cycle -> Integer,
        score -> Double,
        player_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    users (id) {
        id -> Integer,
        user_type -> UserType,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        gender -> Gender,
        #[max_length = 50]
        os -> Nullable<Varchar>,
        player_id -> Integer,
        longitude -> Nullable<Double>,
        latitude -> Nullable<Double>,
        year_of_birth -> Integer,
        #[max_length = 150]
        role_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    roles (name) {
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    permissions (name) {
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    admin_permissions(admin_id, permission_id) {
        admin_id -> Integer,
        #[max_length = 50]
        permission_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    role_permissions (id) {
        id -> Integer,
        #[max_length = 150]
        role_id -> Varchar,
        #[max_length = 50]
        permission_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    sessions (created_at, user_id) {
        created_at -> Timestamp,
        user_id -> Integer,
        times -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    tips (id) {
        id -> Integer,
        #[max_length = 500]
        content -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    player_tips (player_id, tip_id) {
        player_id -> Integer,
        tip_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    events (id) {
        id -> Integer,
        event_type -> EventType,
        #[max_length = 500]
        content -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    functions (id) {
        id -> Integer,
        function_type -> FunctionType,
        event_id -> Integer,
        #[max_length = 50]
        key -> Nullable<Varchar>,
        #[max_length = 50]
        function -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    values (statistic_id, function_id) {
        statistic_id -> Integer,
        function_id -> Integer,
        #[max_length = 200]
        content -> Varchar,
    }
}

diesel::joinable!(users -> roles (role_id));
diesel::joinable!(admins -> roles (role_id));
diesel::joinable!(insurance -> loans (loan_id));
diesel::joinable!(loans -> players (player_id));
diesel::joinable!(plots -> crop_types (crop_type_id));
diesel::joinable!(plots -> players (player_id));
diesel::joinable!(statistics -> players (player_id));
diesel::joinable!(users -> players (player_id));
diesel::joinable!(admin_permissions -> admins (admin_id));
diesel::joinable!(admin_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(player_tips -> players (player_id));
diesel::joinable!(player_tips -> tips (tip_id));
diesel::joinable!(functions -> events (event_id));
diesel::joinable!(values -> functions (function_id));
diesel::joinable!(values -> statistics (statistic_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    crop_types,
    insurance,
    loans,
    players,
    plots,
    statistics,
    users,
    roles,
    admin_permissions,
    permissions,
    role_permissions,
    sessions,
    tips,
    player_tips,
    functions,
    events,
    values,
);
