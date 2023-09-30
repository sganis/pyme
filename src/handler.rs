use std::sync::Arc;
//use jsonwebtoken::{encode, Header};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::Row;
use sqlx::{
    Execute,
    query_builder::QueryBuilder
};
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{
    auth::{Claims, AuthPayload, AuthError, KEYS},
    model::ItemModel,
    schema::{Params, CreateItemSchema, UpdateItemSchema},
    AppState,    
};



pub async fn token(
    Json(payload): Json<AuthPayload>,
    //Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AuthError> {
    // check if email or password is a blank string
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // get the user for the email from database
    // let user = sqlx::query_as::<_, models::auth::User>(
    //     "SELECT username, password FROM users where email = $1",
    // )
    // .bind(&payload.email)
    // .fetch_optional(&pool)
    // .await
    // .map_err(|err| {
    //     dbg!(err);
    //     AuthError::InternalServerError
    // })?;

    // if password is encrypted than decode it first before comparing
    if payload.username != "alice" || payload.password != "secret" {
        // password is incorrect
        Err(AuthError::WrongCredentials)
    } else {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiration = epoch + Duration::from_secs(60 * 60 * 24 * 7); // a week
        let claims = Claims {
            sub: payload.username.to_owned(),
            exp: expiration.as_secs(),
        };
        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;
        // return bearer token
        Ok(Json(json!({ 
            "access_token": token, 
            "type": "Bearer", 
            "username": payload.username 
        })))
    }
}


pub async fn get_items(
    _claims: Claims,
    params: Option<Query<Params>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(params) = params.unwrap_or_default();
    let q = params.q.unwrap_or("".to_string());
    let sortcol = params.sortcol.unwrap_or("date".to_string());
    let desc = params.desc.unwrap_or(true);
    let limit = params.limit.unwrap_or(100);
    let offset = (params.page.unwrap_or(1) - 1) * limit;  

    let mut query = QueryBuilder::new("SELECT * from pyme");
    if q.len() > 0 {
        query.push(" where customer ilike ");
        query.push_bind(format!("%{}%", q));
    }
    query.push(" order by ");
    query.push_bind(sortcol.clone());
    if desc {
        query.push(" desc");
    } else {
        query.push(" asc");
    }
    query.push(" limit ");
    query.push_bind(limit);
    query.push(" offset ");
    query.push_bind(offset);
    let query = query.build_query_as::<ItemModel>();    
    println!("{}", query.sql());
    println!("q={} sortcol={} desc={} limit={} offset={}", q,sortcol,desc,limit,offset);

    let query = query.fetch_all(&data.db).await; 

    match query {
        Ok(items) => {
            Ok(Json(json!(items)))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching all item items"
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    }
}

pub async fn get_item(
    _claims: Claims,
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("id={}", id);
    let query = sqlx::query_as::<_, ItemModel>(r#"
        SELECT * FROM pyme WHERE id = $1
        "#)
        .bind(id)
        .fetch_one(&data.db).await;

    match query {
        Ok(item) => {
            return Ok(Json(json!(item)));
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error)));
        }
    }
}

pub async fn create_item(
    _claims: Claims,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_,ItemModel>(
        "INSERT INTO pyme (date,customer,product,quantity,price,paid) 
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(body.date.to_string())
        .bind(body.customer.to_string())
        .bind(body.product.to_string())
        .bind(body.quantity)
        .bind(body.price)
        .bind(body.paid)
        .fetch_one(&data.db)
        .await;

    match query {
        Ok(item) => {
            let res = json!(item);
            return Ok((StatusCode::CREATED, Json(res)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error = json!({
                    "detail": "Item with that title already exists",
                });
                println!("{:?}", e);
                return Err((StatusCode::CONFLICT, Json(error)));
            }
            println!("{:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn update_item(
    _claims: Claims, 
    Path(id): Path<i32>, 
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_, ItemModel>(
        "SELECT * FROM items WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await;

    if query.is_err() {
        let error = json!({
            "datail": format!("Item with ID: {} not found", id)
        });
        println!("{:?}", query);
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    let item = query.unwrap();

    let query = sqlx::query_as::<_, ItemModel>(
        "UPDATE items SET date=$1, customer=$2, product=$3, 
            quantity=$4, price=$5 WHERE id=$6 RETURNING *")
        .bind(body.date.to_owned().unwrap_or(item.date))
        .bind(body.customer.to_owned().unwrap_or(item.customer))
        .bind(body.product.to_owned().unwrap_or(item.product))
        .bind(body.quantity.to_owned().unwrap_or(item.quantity))
        .bind(body.price.to_owned().unwrap_or(item.quantity))
        .bind(id)
        .fetch_one(&data.db)
        .await;

    match query {
        Ok(item) => {
            return Ok(Json(serde_json::json!(item)));
        }
        Err(e) => {
            println!("{:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn delete_item(
    _claims: Claims, 
    Path(id): Path<i32>,  
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query("DELETE FROM items  WHERE id = $1")
        .bind(id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error = json!({
            "detail": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    Ok(StatusCode::NO_CONTENT)
}


pub async fn get_customers(
    _claims: Claims, 
    params: Option<Query<Params>>, 
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //println!("{:?}", params);
    let Query(params) = params.unwrap_or_default();
    println!("{:?}", params);
    let letters = params.q.unwrap_or("".to_string());
    let query;

    if letters.len() > 0 {
        query = sqlx::query(r#"
            SELECT DISTINCT customer FROM pyme 
            WHERE customer ilike $1 
            ORDER BY customer
            "#).bind(format!("%{}%", letters));
    } else {
        query = sqlx::query(r#"
            SELECT DISTINCT customer FROM pyme ORDER BY customer
            "#);
    }
    let query = query.fetch_all(&data.db).await; 

    match query {
        Ok(records) => {
            let mut customers = Vec::<String>::new();
            for rec in records {
                customers.push(rec.get(0));
            }
            Ok(Json(customers))        
        },
        Err(e) => {
            println!("error: {:?}", e);
            let error = serde_json::json!({
                "detail": "Something bad happened while fetching customers",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    } 
}

pub async fn get_stat_customer(
    _claims: Claims, 
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(r#"
        select customer, cast(sum(quantity) as text), cast(sum(price) as text)
        from public.pyme
        group by customer 
        order by sum(price) desc
        "#)
        .fetch_all(&data.db).await; 

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec::<String>>::new();
            for rec in records {
                let mut r = Vec::<String>::new();
                r.push(rec.get(0));
                r.push(rec.get(1));
                r.push(rec.get(2));
                rows.push(r);
            }
            Ok(Json(rows))        
        },
        Err(e) => {
            println!("error: {:?}", e);
            let error = serde_json::json!({
                "detail": "Something bad happened while fetching customers",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    } 
}
pub async fn get_stat_product(
    _claims: Claims, 
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(r#"
        select product, cast(sum(quantity) as text), cast(sum(price) as text) 
        from public.pyme
        group by product 
        order by sum(price) desc
        "#)
        .fetch_all(&data.db).await; 

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec::<String>>::new();
            for rec in records {
                let mut r = Vec::<String>::new();
                r.push(rec.get(0));
                r.push(rec.get(1));
                r.push(rec.get(2));
                rows.push(r);
            }
            Ok(Json(rows))        
        },
        Err(e) => {
            println!("error: {:?}", e);
            let error = serde_json::json!({
                "detail": "Something bad happened while fetching customers",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    } 
}

pub async fn get_stat_year(
    _claims: Claims, 
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(r#"
        select cast(year as text), cast(sum(quantity) as text), cast(sum(price) as text)
        from (
            select to_date(date, 'YYYY-MM-DD') as date 
                ,extract(year from to_date(date, 'YYYY-MM-DD')) as year
                ,customer
                ,product
                ,quantity
                ,price
                from public.pyme
            ) as t
        group by year order by year
        "#)
        .fetch_all(&data.db).await; 

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec::<String>>::new();
            for rec in records {
                let mut r = Vec::<String>::new();
                r.push(rec.get(0));
                r.push(rec.get(1));
                r.push(rec.get(2));
                rows.push(r);
            }
            Ok(Json(rows))        
        },
        Err(e) => {
            println!("error: {:?}", e);
            let error = serde_json::json!({
                "detail": "Something bad happened while fetching customers",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    } 
}

pub async fn get_stat_quarter(
    _claims: Claims, 
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(r#"
        select cast(quarter as text), cast(sum(quantity) as text), cast(sum(price) as text)
        from (
            select to_date(date, 'YYYY-MM-DD') as date 
                ,extract(year from to_date(date, 'YYYY-MM-DD')) as year
                ,extract(year from to_date(date, 'YYYY-MM-DD')) ||'/Q'|| extract(quarter from to_date(date, 'YYYY-MM-DD')) as quarter
                ,customer
                ,product
                ,quantity
                ,price
                from public.pyme
            ) as t
        group by quarter order by quarter
        "#)
        .fetch_all(&data.db).await; 

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec::<String>>::new();
            for rec in records {
                let mut r = Vec::<String>::new();
                r.push(rec.get(0));
                r.push(rec.get(1));
                r.push(rec.get(2));
                rows.push(r);
            }
            Ok(Json(rows))        
        },
        Err(e) => {
            println!("error: {:?}", e);
            let error = serde_json::json!({
                "detail": "Something bad happened while fetching customers",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
        }
    } 
}





