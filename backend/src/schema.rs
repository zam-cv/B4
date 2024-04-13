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
        current_cycle -> Integer,
        current_score -> Double,
        balance_cash -> Integer,
        balance_verqor -> Integer,
        balance_coyote -> Integer,
        max_plots -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    plots (id) {
        id -> Integer,
        #[max_length = 50]
        crop_type_id -> Nullable<Varchar>,
        player_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    statistics (id) {
        id -> Integer,
        cycle -> Integer,
        score -> Integer,
        player_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    users (id) {
        id -> Integer,
        #[max_length = 13]
        user_type -> UserType,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 150]
        password -> Varchar,
        #[max_length = 1]
        gender -> Gender,
        #[max_length = 50]
        os -> Nullable<Varchar>,
        player_id -> Integer,
        longitude -> Nullable<Double>,
        latitude -> Nullable<Double>,
        year_of_birth -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    roles (id) {
        id -> Integer,
        name -> RoleType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    user_roles(id) {
        id -> Integer,
        user_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    admin_roles(id) {
        id -> Integer,
        admin_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    permissions (id) {
        id -> Integer,
        name -> PermissionType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::models::types::exports::*;

    role_permissions (id) {
        id -> Integer,
        role_id -> Integer,
        permission_id -> Integer,
    }
}

diesel::joinable!(insurance -> loans (loan_id));
diesel::joinable!(loans -> players (player_id));
diesel::joinable!(plots -> crop_types (crop_type_id));
diesel::joinable!(plots -> players (player_id));
diesel::joinable!(statistics -> players (player_id));
diesel::joinable!(users -> players (player_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(admin_roles -> admins (admin_id));
diesel::joinable!(admin_roles -> roles (role_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));

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
    user_roles,
    admin_roles,
    permissions,
    role_permissions,
);
