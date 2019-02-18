table! {
    airplanes {
        id -> Nullable<Integer>,
        name  -> Text,
        description  -> Text,
        year_in_service -> Integer,
        country_of_origin -> Text,
        operators -> Text,
        max_speed -> Integer,
        max_range -> Integer,
        ceiling -> Integer,
        engines -> Text,
        img_url -> Text,
    }
}

table! {
    favorites {
        id -> Nullable<Integer>,
        airplane_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
    }
}
