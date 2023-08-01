// @generated automatically by Diesel CLI.

diesel::table! {
    user_logins (id) {
        id -> Int4,
        user_id -> Int4,
        device_id -> Varchar,
        ip_address -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        guid -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        phone -> Nullable<Varchar>,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(user_logins -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_logins,
    users,
);
