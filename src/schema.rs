// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Nullable<Integer>,
        first_name -> Nullable<Text>,
        diplomacy -> Nullable<Integer>,
        martial -> Nullable<Integer>,
        stewardship -> Nullable<Integer>,
        intrigue -> Nullable<Integer>,
        learning -> Nullable<Integer>,
        prowess -> Nullable<Integer>,
        faith_id -> Nullable<Integer>,
        house_id -> Nullable<Integer>,
    }
}

diesel::table! {
    faiths (id) {
        id -> Nullable<Integer>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    houses (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(characters -> faiths (faith_id));
diesel::joinable!(characters -> houses (house_id));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    faiths,
    houses,
);
