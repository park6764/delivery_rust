use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Selectable, Queryable, Identifiable, Serialize, Clone)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub id: i32,
    pub username: String,
    pub pw_hash: String,
    pub addr: String,
    pub token: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::customers)]
pub struct CreateCustomer {
    pub username: String,
    pub pw_hash: String,
    pub addr: String,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Associations)]
#[diesel(table_name = crate::schema::menus)]
#[diesel(belongs_to(Restaurant, foreign_key=restid))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Menu {
    pub id: i32,
    #[diesel(column_name=menuname)]
    pub menu_name: String,
    pub price: i32,
    #[diesel(column_name=restid)]
    pub rest_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::menus)]
pub struct CreateMenu {
    #[diesel(column_name=menuname)]
    pub menu_name: String,
    pub price: i32,
    #[diesel(column_name=restid)]
    pub rest_id: i32,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Associations)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(belongs_to(Customer, foreign_key=userid))]
#[diesel(belongs_to(Restaurant, foreign_key=restid))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub menus: Vec<Option<i32>>,
    #[diesel(column_name=userid)]
    pub user_id: i32,
    #[diesel(column_name=restid)]
    pub rest_id: i32,
    #[diesel(column_name=orderedat)]
    pub ordered_at: SystemTime
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::orders)]
pub struct CreateOrder {
    pub menus: Vec<Option<i32>>,
    #[diesel(column_name=userid)]
    pub user_id: i32,
    #[diesel(column_name=restid)]
    pub rest_id: i32,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Clone)]
#[diesel(table_name = crate::schema::owners)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Owner {
    pub id: i32,
    #[diesel(column_name=ownername)]
    pub owner_name: String,
    pub pw_hash: String,
    pub token: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::owners)]
pub struct CreateOwner {
    #[diesel(column_name=ownername)]
    pub owner_name: String,
    pub pw_hash: String,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Associations)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(belongs_to(Owner, foreign_key=ownerid))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Restaurant {
    pub id: i32,
    #[diesel(column_name=ownerid)]
    pub owner_id: i32,
    #[diesel(column_name=restname)]
    pub rest_name: String,
    #[diesel(column_name=totalsales)]
    pub total_sales: Option<i64>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct CreateRestaurant {
    #[diesel(column_name=ownerid)]
    pub owner_id: i32,
    #[diesel(column_name=restname)]
    pub rest_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: i32,
    pub is_owner: bool
}

#[derive(Deserialize)]
pub struct AddressForm {
    pub id: i32,
    pub addr: String
}

#[derive(Deserialize)]
pub struct QueryId {
    pub id: i32
}