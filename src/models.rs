use diesel::prelude::*;
use crate::schema::{characters, faiths, houses};

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Faith))]
#[diesel(belongs_to(House))]
#[diesel(table_name = characters)]
pub struct Character {
    pub id: i32,
    pub first_name: Option<String>,
    pub faith_id: Option<i32>,
    pub house_id: Option<i32>,
    pub diplomacy: i32,
    pub martial: i32,
    pub stewardship: i32,
    pub intrigue: i32,
    pub learning: i32,
    pub prowess: i32,
}

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter {
    pub id: i32,
    pub first_name: Option<String>,
    pub prowess: i32,
    pub faith_id: Option<i32>,
    pub house_id: Option<i32>,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = faiths)]
pub struct Faith {
    pub id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[diesel(table_name = faiths)]
pub struct NewFaith {
    pub id: i32,
    pub tag: String,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = houses)]
pub struct House {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = houses)]
pub struct NewHouse {
    pub id: i32,
    pub name: Option<String>,
}