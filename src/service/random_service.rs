use crate::database::dto::place::Place;
use chrono::Utc;

pub struct RandomCityService {}

impl RandomCityService {
    pub fn new() -> RandomCityService {
        RandomCityService {}
    }

    pub fn get_random_parking(&self, city: &str) -> Vec<Place> {
        if city == "paris" {
            return vec![
                Place::new(1, 50.631778, 3.045893, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621754, 3.036441, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619691, 3.039200, 1, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        } else if city == "lille" {
            return vec![
                Place::new(1, 50.631778, 3.045893, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621754, 3.036441, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619691, 3.039200, 2, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        }
        return vec![
            Place::new(1, 50.631778, 3.045893, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(2, 50.621754, 3.036441, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(3, 50.619691, 3.039200, 3, Utc::now().naive_utc(), Utc::now().naive_utc())
        ];
    }
}
