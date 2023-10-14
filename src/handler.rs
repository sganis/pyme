use crate::{
    auth::{AuthData, AuthError, Claims, KEYS},
    model::{CountModel, CreateOrderSchema, OrderModel, OrderSchema, Params},
    AppState,
};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use jsonwebtoken::{encode, Header};
use serde_json::json;
use sqlx::Row;
use sqlx::{query_builder::QueryBuilder, Execute};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub async fn token(
    State(state): State<Arc<AppState>>,
    Json(authdata): Json<AuthData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // check if email or password is a blank string
    if authdata.username.is_empty() || authdata.password.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ));
    }

    let query = sqlx::query(
        "
        SELECT username,password_hash FROM public.user WHERE username = $1
        ",
    )
    .bind(authdata.username.clone())
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
        println!("error: {:?}", e);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ));
    }

    let hash_password: String = query.unwrap().get(1);
    // println!("hash = {}", hash_password.clone());
    // println!("password={}", authdata.password.clone());
    let db_hash = PasswordHash::new(&hash_password).unwrap();
    let valid: bool = Argon2::default()
        .verify_password(authdata.password.as_bytes(), &db_hash)
        .is_ok();

    if !valid {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ))
    } else {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiration = epoch + Duration::from_secs(60 * 60 * 24 * 7); // a week
        let claims = Claims {
            sub: authdata.username.to_owned(),
            exp: expiration.as_secs(),
        };
        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)
            .unwrap();

        Ok(Json(json!({
            "access_token": token,
            "type": "Bearer",
            "username": authdata.username
        })))
    }
}

pub async fn password(
    State(state): State<Arc<AppState>>,
    Json(authdata): Json<AuthData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let newpassword = authdata.newpassword.unwrap();
    if authdata.username.is_empty()
        || authdata.password.is_empty()
        || newpassword.is_empty()
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ));
    }

    let query = sqlx::query(
        "
        SELECT username,password_hash FROM public.user WHERE username = $1
        ",
    )
    .bind(authdata.username.clone())
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
        println!("error: {:?}", e);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ));
    }

    let hash_password: String = query.unwrap().get(1);
    let db_hash = PasswordHash::new(&hash_password).unwrap();
    let valid: bool = Argon2::default()
        .verify_password(authdata.password.as_bytes(), &db_hash)
        .is_ok();

    if !valid {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"detail": "Wrong credentials"})),
        ))
    } else {
        // change password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(newpassword.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let query = sqlx::query(
            "
            UPDATE public.user SET password_hash=$1
            WHERE username=$2 returning *
            ",
        )
        .bind(password_hash.clone())
        .bind(authdata.username.clone())
        .fetch_one(&state.db)
        .await;

        if let Err(e) = query {
            println!("{e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "Password not changed"})),
            ))
        } else {
            Ok(Json(json!({"result": "Password changed successfully"})))
        }
    }
}

pub async fn get_items(
    _claims: Claims,
    params: Option<Query<Params>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(params) = params.unwrap_or_default();
    let q = params.q.unwrap_or("".to_string());
    let sortcol = params.sortcol.unwrap_or("date".to_string());
    let desc = params.desc.unwrap_or(true);
    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);

    let mut query = QueryBuilder::new(
        "SELECT count(*) from pyme_order where deleted=false",
    );
    if !q.is_empty() {
        query.push(" and customer ilike ");
        query.push_bind(format!("%{}%", q));
    }
    let query_total = query
        .build_query_as::<CountModel>()
        .fetch_one(&state.db)
        .await;

    if let Err(e) = query_total {
        println!("{e}");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "detail": "Something bad happened while fetching all item items"
            })),
        ));
    }
    let total = query_total.unwrap().count;
    println!("total={total}");

    query = QueryBuilder::new("SELECT * from pyme_order where deleted=false");
    if !q.is_empty() {
        query.push(" and customer ilike ");
        query.push_bind(format!("%{}%", q));
    }
    query.push(" order by ");
    query.push(sortcol.clone());
    if desc {
        query.push(" desc");
    } else {
        query.push(" asc");
    }
    query.push(", id desc ");
    query.push(" limit ");
    query.push_bind(limit);
    query.push(" offset ");
    query.push_bind(offset);
    let query = query.build_query_as::<OrderModel>();
    println!("{}", query.sql());
    println!(
        "q={} sortcol={} desc={} limit={} offset={}",
        q, sortcol, desc, limit, offset
    );

    let query = query.fetch_all(&state.db).await;

    match query {
        Ok(items) => {
            // println!("{:?}", items[0]);
            Ok(Json(json!({
                "items" : items,
                "total" : total,
                "limit" : limit,
                "offset" : offset,
            })))
        }
        Err(e) => {
            println!("error: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "detail": "Something bad happened while fetching all item items"
                })),
            ))
        }
    }
}

pub async fn get_item(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("id={}", id);
    let query = sqlx::query_as::<_, OrderModel>(
        "SELECT * FROM pyme_order WHERE id = $1 and deleted=false",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
        println!("error: {:?}", e);
        let error = json!({
            "detail": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    let order = query.unwrap();

    // let query = sqlx::query_as::<_, ItemModel>(
    //     "SELECT * FROM pyme_order_item WHERE order_id = $1 and deleted=false",
    // )
    // .bind(order.id)
    // .fetch_all(&state.db)
    // .await;

    // if let Err(e) = query {
    //     println!("error: {:?}", e);
    //     let error = json!({
    //         "detail": format!("Items for order {} not found", id)
    //     });
    //     return Err((StatusCode::NOT_FOUND, Json(error)));
    // }
    // let mut order = json!(order);
    // let items = json!(query.unwrap());
    // order["items"] = items;
    Ok(Json(order))
}

pub async fn create_item(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateOrderSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_, OrderModel>(
        "INSERT INTO pyme_order (date,customer,price,paid,notes,items,username) 
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(body.date.to_string())
    .bind(body.customer.to_string())
    .bind(body.price)
    .bind(body.paid)
    .bind(body.notes)
    .bind(json!(body.items))
    .bind(claims.sub.clone())
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
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
    let order = query.unwrap();
    // let mut items = Vec::<ItemModel>::new();

    // for i in body.items.into_iter() {
    //     let query = sqlx::query_as::<_, ItemModel>(
    //         "INSERT INTO pyme_order_item (order_id,product,quantity,price,username)
    //             VALUES ($1, $2, $3, $4, $5) RETURNING *",
    //     )
    //     .bind(order.id)
    //     .bind(i.product)
    //     .bind(i.quantity)
    //     .bind(i.price)
    //     .bind(claims.sub.clone())
    //     .fetch_one(&state.db)
    //     .await;
    //     if let Err(e) = query {
    //         println!("{:?}", e);
    //         let error =
    //             json!({"status": "error","message": format!("{:?}", e)});
    //         return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
    //     }
    //     items.push(query.unwrap());
    // }
    // let mut order = json!(order);
    // order["items"] = json!(items);
    // //println!("order created: {:?}", order);
    Ok(Json(order))
}

pub async fn update_item(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(body): Json<OrderSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_, OrderModel>(
        "SELECT * FROM pyme_order WHERE id = $1",
    )
    .bind(body.id)
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
        println!("{:?}", e);
        return Err((
            StatusCode::NOT_FOUND,
            Json(
                json!({"datail": format!("Item with ID: {} not found", body.id)}),
            ),
        ));
    }

    let order = query.unwrap();

    let query = sqlx::query_as::<_, OrderModel>(
        "UPDATE pyme_order SET 
            date=$1, customer=$2, price=$3, paid=$4, 
            notes=$5, items=$6, username=$7
            WHERE id=$8 and deleted=false RETURNING *",
    )
    .bind(body.date.unwrap_or(order.date.clone()))
    .bind(body.customer.unwrap_or(order.customer.clone()))
    .bind(body.price.unwrap_or(order.price))
    .bind(body.paid.unwrap_or(order.paid))
    .bind(body.notes.unwrap_or(order.notes.clone()))
    .bind(json!(body.items.unwrap_or(
        serde_json::from_value(json!(order.items)).unwrap()
    )))
    .bind(claims.sub.clone())
    .bind(body.id)
    .fetch_one(&state.db)
    .await;

    if let Err(e) = query {
        println!("{:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"detail": format!("{:?}", e)})),
        ));
    }

    // let rows_affected = sqlx::query(
    //     "UPDATE pyme_order_item SET deleted=true, username=$1
    //         WHERE order_id = $2 and deleted=false",
    // )
    // .bind(claims.sub.clone())
    // .bind(order.id)
    // .execute(&state.db)
    // .await
    // .unwrap()
    // .rows_affected();

    // if rows_affected == 0 {
    //     return Err((
    //         StatusCode::NOT_FOUND,
    //         Json(json!({
    //             "detail": format!("Item with ID: {} not found", order.id)
    //         })),
    //     ));
    // }

    // let mut items = Vec::<ItemModel>::new();

    // for i in body.items.unwrap().into_iter() {
    //     let query = sqlx::query_as::<_, ItemModel>(
    //         "INSERT INTO pyme_order_item (order_id,product,quantity,price,username)
    //             VALUES ($1, $2, $3, $4, $5) RETURNING *",
    //     )
    //     .bind(order.id)
    //     .bind(i.product)
    //     .bind(i.quantity)
    //     .bind(i.price)
    //     .bind(claims.sub.clone())
    //     .fetch_one(&state.db)
    //     .await;
    //     if let Err(e) = query {
    //         return Err((
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             Json(json!({"status": "error","message": format!("{:?}", e)})),
    //         ));
    //     }
    //     items.push(query.unwrap());
    // }
    // let mut order = json!(order);
    // order["items"] = json!(items);
    Ok(Json(order))
}

pub async fn delete_item(
    claims: Claims,
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_, OrderModel>(
        "SELECT * FROM pyme_order WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await;
    if let Err(e) = query {
        println!("{:?}", e);
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"datail": format!("Item with ID: {} not found", id)})),
        ));
    }
    let order = query.unwrap();
    let rows_affected = sqlx::query(
        "UPDATE pyme_order SET deleted=true, username=$1 WHERE id=$2 and deleted=false",
    )
    .bind(claims.sub.clone())
    .bind(order.id)
    .execute(&state.db)
    .await
    .unwrap()
    .rows_affected();
    if rows_affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "detail": format!("Item with ID: {} not found", order.id)
            })),
        ));
    }
    // let rows_affected = sqlx::query(
    //     "UPDATE pyme_order_item
    //         SET deleted=true, user=$1
    //         WHERE order_id=$2 and deleted=false",
    // )
    // .bind(claims.sub.clone())
    // .bind(order.id)
    // .execute(&state.db)
    // .await
    // .unwrap()
    // .rows_affected();
    // if rows_affected == 0 {
    //     return Err((
    //         StatusCode::NOT_FOUND,
    //         Json(json!({
    //             "detail": format!("Items with ID: {} not found", order.id)
    //         })),
    //     ));
    // }
    Ok(Json(json!({"result": "ok"})))
}

pub async fn get_customers(
    _claims: Claims,
    params: Option<Query<Params>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    //println!("{:?}", params);
    let Query(params) = params.unwrap_or_default();
    println!("{:?}", params);
    let letters = params.q.unwrap_or("".to_string());

    let query = if !letters.is_empty() {
        sqlx::query(
            "
            SELECT DISTINCT customer FROM pyme_order
            WHERE customer ilike $1 
            ORDER BY customer
            ",
        )
        .bind(format!("%{}%", letters))
    } else {
        sqlx::query(
            "
            SELECT DISTINCT customer FROM pyme_order ORDER BY customer
            ",
        )
    };
    let query = query.fetch_all(&state.db).await;

    match query {
        Ok(records) => {
            let mut customers = Vec::<String>::new();
            for rec in records {
                customers.push(rec.get(0));
            }
            Ok(Json(customers))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching customers",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

pub async fn get_products(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(
        "select cast(value as text) 
        from pyme_config where key='products'
        ",
    )
    .fetch_one(&state.db)
    .await;

    match query {
        Ok(record) => {
            // let mut rows = Vec::<Vec<String>>::new();
            // for rec in records {
            //     let r = vec![rec.get(0), rec.get(1)];
            //     rows.push(r);
            // }
            let products: String = record.get(0);
            // string, converted to json in frontend
            Ok(Json(products))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching products",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

pub async fn get_stat_customer(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(
        r#"
        -- top customer
        select customer, cast(units as text), cast(price as text) from (
            select customer
            ,sum((i->>'quantity')::int) as units
            ,sum((i->>'price')::int) as price 
            from (select customer, json_array_elements(items) as i 
                  from pyme_order 
                  where not deleted
                 ) x
            group by customer
            order by price desc
        ) y        
        "#,
    )
    .fetch_all(&state.db)
    .await;

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec<String>>::new();
            for rec in records {
                let r = vec![rec.get(0), rec.get(1), rec.get(2)];
                rows.push(r);
            }
            Ok(Json(rows))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching customers",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}
pub async fn get_stat_product(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(
        r#"
        -- top product
        select product, cast(units as text), cast(price as text) from (
        select i->>'product' as product
        ,sum((i->>'quantity')::int) as units
        ,sum((i->>'price')::int) as price
        from (select json_array_elements(items) as i
            from pyme_order
            where not deleted) x
        group by i->>'product'
        order by sum((i->>'price')::int) desc
        ) y
        "#,
    )
    .fetch_all(&state.db)
    .await;

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec<String>>::new();
            for rec in records {
                let r = vec![rec.get(0), rec.get(1), rec.get(2)];
                rows.push(r);
            }
            Ok(Json(rows))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching customers",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

pub async fn get_stat_year(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query(
        "
        select cast(year as text), cast(units as text), cast(price as text) from (
            select year
            ,sum((i->>'quantity')::int) as units
            ,sum((i->>'price')::int) as price 
            from (select extract(year from to_date(date, 'YYYY-MM-DD')) as year
                  ,json_array_elements(items) as i 
                  from pyme_order where not deleted) x
            group by year order by year desc
            ) y
            
        ",
    )
    .fetch_all(&state.db)
    .await;

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec<String>>::new();
            for rec in records {
                let r = vec![rec.get(0), rec.get(1), rec.get(2)];
                rows.push(r);
            }
            Ok(Json(rows))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching customers",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}

pub async fn get_stat_quarter(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query("
        -- top quarter
        select cast(quarter as text), cast(units as text), cast(price as text) from (
        select quarter
        ,sum((i->>'quantity')::int) as units
        ,sum((i->>'price')::int) as price 
        from (select 
            extract(year from to_date(date, 'YYYY-MM-DD')) 
                ||'/Q'|| extract(quarter from to_date(date, 'YYYY-MM-DD')) 
                as quarter
            ,json_array_elements(items) as i 
            from pyme_order where not deleted) x
        group by quarter order by quarter desc
        ) y
        ")
        .fetch_all(&state.db).await;

    match query {
        Ok(records) => {
            let mut rows = Vec::<Vec<String>>::new();
            for rec in records {
                let r = vec![rec.get(0), rec.get(1), rec.get(2)];
                rows.push(r);
            }
            Ok(Json(rows))
        }
        Err(e) => {
            println!("error: {:?}", e);
            let error = json!({
                "detail": "Something bad happened while fetching customers",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
        }
    }
}
