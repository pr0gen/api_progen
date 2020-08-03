table! {
    city (id) {
        id -> Integer,
        name -> Varchar,
        postal_code -> Integer,
        country_id -> Integer,
    }
}

table! {
    country (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    place (id) {
        id -> Integer,
        longitude -> Float,
        latitude -> Float,
        city_id -> Integer,
        nb_place -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(city -> country (country_id));
joinable!(place -> city (city_id));

allow_tables_to_appear_in_same_query!(city, country, place,);
