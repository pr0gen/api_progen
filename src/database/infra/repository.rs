use crate::database::dto::Dto;
use diesel::prelude::*;

pub trait Repository<'a, C: Connection, D: Dto> {
    
    fn new(connection: &'a C) -> Self;
    
    fn select(&self) -> Vec<D>;
    
    fn select_by_id(&self, idp: i32) -> Vec<D>;
    
    fn insert(&self, data: D) -> QueryResult<usize>;  
}

