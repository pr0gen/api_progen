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
                Place::new(1, 50.631_78, 3.045_89, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621_78, 3.036_44, 1, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619_69, 3.039_20, 1, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        } else if city == "lille" {
            return vec![
                Place::new(1, 50.631_77, 3.045_89, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(2, 50.621_75, 3.036_44, 2, Utc::now().naive_utc(), Utc::now().naive_utc()),
                Place::new(3, 50.619_69, 3.039_20, 2, Utc::now().naive_utc(), Utc::now().naive_utc())
            ];
        }
        return vec![
            Place::new(1, 50.631_77, 3.045_83, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(2, 50.621_75, 3.036_44, 3, Utc::now().naive_utc(), Utc::now().naive_utc()),
            Place::new(3, 50.619_69, 3.039_20, 3, Utc::now().naive_utc(), Utc::now().naive_utc())
        ];
    }
}
