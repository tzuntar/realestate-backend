// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        registration_date -> Nullable<Timestamp>,
    }
}
