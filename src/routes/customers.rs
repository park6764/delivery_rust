use axum::{extract::{State, Query}, Json, http::StatusCode, Extension};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::models::{CreateCustomer, AddressForm, Owner, Customer, QueryId};

use super::internal_error;

pub async fn create_customer(
    State(pool): State<Pool>,
    Json(customer): Json<CreateCustomer>
) -> Result<(), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    use crate::schema::customers::dsl::*;

    let _ = conn.interact(|conn| {
        diesel::insert_into(customers)
            .values(customer)
            .execute(conn)
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    Ok(())
}

pub async fn change_address(
    State(pool): State<Pool>,
    Json(form): Json<AddressForm>
) -> Result<(), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    use crate::schema::customers::dsl::*;

    let _ = conn.interact(move |conn| {
        diesel::update(customers.find(form.id))
        .set(addr.eq(form.addr))
        .execute(conn)
    }).await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    Ok(())
}

pub async fn delete_user(
    State(pool): State<Pool>,
    Extension(user): Extension<Result<Owner, Customer>>,
) -> Result<(), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    match user {
        Ok(o) => {
            use crate::schema::owners::dsl::*;

            let _ = conn.interact(move |conn| {
                diesel::delete(owners.find(o.id))
                .execute(conn)
            }).await
            .map_err(internal_error)?
            .map_err(internal_error)?;
            
            Ok(())
        },
        Err(c) => {
            use crate::schema::customers::dsl::*;

            let _ = conn.interact(move |conn| {
                diesel::delete(customers.find(c.id))
                .execute(conn)
            }).await
            .map_err(internal_error)?
            .map_err(internal_error)?;
            
            Ok(())
        }
    }
}