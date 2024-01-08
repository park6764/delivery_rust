// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 5]
        username -> Varchar,
        #[max_length = 10]
        pw_hash -> Varchar,
        #[max_length = 20]
        addr -> Varchar,
        token -> Nullable<Varchar>,
    }
}

diesel::table! {
    menus (id) {
        id -> Int4,
        #[max_length = 10]
        menuname -> Varchar,
        price -> Int4,
        restid -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        menus -> Array<Nullable<Int4>>,
        userid -> Int4,
        restid -> Int4,
        orderedat -> Timestamp,
    }
}

diesel::table! {
    owners (id) {
        id -> Int4,
        #[max_length = 5]
        ownername -> Varchar,
        #[max_length = 10]
        pw_hash -> Varchar,
        token -> Nullable<Varchar>,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Int4,
        ownerid -> Int4,
        #[max_length = 10]
        restname -> Varchar,
        totalsales -> Nullable<Int8>,
    }
}

diesel::table! {
    todo (id) {
        id -> Int4,
        #[max_length = 15]
        title -> Varchar,
        body -> Text,
        complete -> Bool,
    }
}

diesel::joinable!(menus -> restaurants (restid));
diesel::joinable!(orders -> customers (userid));
diesel::joinable!(orders -> restaurants (restid));
diesel::joinable!(restaurants -> owners (ownerid));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    menus,
    orders,
    owners,
    restaurants,
    todo,
);
