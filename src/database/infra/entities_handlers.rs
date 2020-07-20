use crate::database::dto::Dto;
use diesel::prelude::*;

pub trait EntityHandler<C: Connection, D: Dto> {
    fn new(connection: C) -> Self;
    fn select(&self) -> Vec<D>;
    fn select_by_id(&self, idp: i32) -> Vec<D>;
}

