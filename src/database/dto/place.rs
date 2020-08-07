use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto;
use crate::database::dto::Dto;
use crate::database::schema::place;

pub fn as_place(longitude: f32, latitude: f32, city_id: i32, nb_place: i32) -> Place {
    Place {
        id: 0,
        longitude,
        latitude,
        city_id,
        nb_place,
        created_at: dto::now(),
        updated_at: dto::now(),
    }
}

#[test]
pub fn should_convert() {
    let expected = Place::new(0, 1.0, 2.0, 3, 4, dto::now(), dto::now());
    let actual = as_place(1.0, 2.0, 3, 4);
    assert_eq!(actual.id, expected.id);
    assert_eq!(actual.longitude, expected.longitude);
    assert_eq!(actual.latitude, expected.latitude);
    assert_eq!(actual.city_id, expected.city_id);
    assert_eq!(actual.nb_place, expected.nb_place);
}

#[derive(Serialize, Queryable, Deserialize)]
pub struct Place {
    id: i32,
    longitude: f32,
    latitude: f32,
    city_id: i32,
    nb_place: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "place"]
pub struct InsertablePlace {
    longitude: f32,
    latitude: f32,
    city_id: i32,
    nb_place: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}


impl Dto for Place {}

impl Place {
    pub fn new(
        id: i32,
        longitude: f32,
        latitude: f32,
        city_id: i32,
        nb_place: i32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Place {
            id,
            longitude,
            latitude,
            city_id,
            nb_place,
            created_at,
            updated_at,
        }
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude
    }

    pub fn get_nb_place(&self) -> i32 {
        self.nb_place
    }
}

impl InsertablePlace {
    pub fn from_place(place: &Place) -> InsertablePlace {
        InsertablePlace {
            longitude: place.longitude,
            latitude: place.latitude,
            city_id: place.city_id,
            nb_place: place.nb_place,
            created_at: place.created_at,
            updated_at: place.updated_at,
        }
    }
}

