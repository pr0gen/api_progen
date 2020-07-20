use crate::database::dto::place::Place;
use chrono::Utc;

pub struct RandomCityService {}

impl Default for RandomCityService {
    fn default() -> Self {
        RandomCityService{}
    }
}

impl RandomCityService {

    pub fn get_random_parking(&self, city: &str) -> Vec<Place> {
        if city == "paris" {
            return vec![
                Place::new(1, 50.631_778, 3.045_893, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621_754, 3.036_441, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619_691, 3.039_200, 1, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        } else if city == "lille" {
            return vec![
                Place::new(1, 50.631_778, 3.045_893, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621_754, 3.036_441, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619_691, 3.039_200, 2, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        }
        return vec![
            Place::new(1, 50.631_778, 3.045_893, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(2, 50.621_754, 3.036_441, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(3, 50.619_691, 3.039_200, 3, Utc::now().naive_utc(), Utc::now().naive_utc())
        ];
    }
}
