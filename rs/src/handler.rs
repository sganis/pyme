use std::sync::Arc;
use serde_json::json;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};


use crate::{
    model::ItemModel,
    schema::{CreateItemSchema, FilterOptions, UpdateItemSchema},
    AppState,
};



pub async fn get_items(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as::<_,ItemModel>(
        "SELECT * FROM pyme ORDER BY id DESC limit $1 offset $2")
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        println!("{:?}", query_result);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn get_item(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as::<_, ItemModel>(
        "SELECT * FROM pyme WHERE id = $1"  
        ).bind(id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                "note": note
            })});
            return Ok(Json(note_response));
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            println!("{:?}", e);
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn create_item(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as::<_, ItemModel>(
        "INSERT INTO pyme (date,customer,product,quantity,price) 
            VALUES ($1, $2, $3, $5, $5) RETURNING *")
        .bind(body.date.to_string())
        .bind(body.customer.to_string())
        .bind(body.product.to_string())
        .bind(body.quantity.to_string())
        .bind(body.price.to_string())
        //.bind(body.category.to_owned().unwrap_or("".to_string())
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({
                "status": "success",
                "data": json!({
                "note": note
            })});
            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Item with that title already exists",
                });
                println!("{:?}", e);
                return Err((StatusCode::CONFLICT, Json(error_response)));
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
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as::<_, ItemModel>(
        "SELECT * FROM items WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        println!("{:?}", query_result);
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let query_result = sqlx::query_as::<_, ItemModel>(
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

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
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
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query("DELETE FROM items  WHERE id = $1")
        .bind(id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}






// pub async fn register_user_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<RegisterUserSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let user_exists: Option<bool> =
//         sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
//             .bind(body.email.to_owned().to_ascii_lowercase())
//             .fetch_one(&data.db)
//             .await
//             .map_err(|e| {
//                 let error_response = serde_json::json!({
//                     "status": "fail",
//                     "message": format!("Database error: {}", e),
//                 });
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//             })?;

//     if let Some(exists) = user_exists {
//         if exists {
//             let error_response = serde_json::json!({
//                 "status": "fail",
//                 "message": "User with that email already exists",
//             });
//             return Err((StatusCode::CONFLICT, Json(error_response)));
//         }
//     }

//     let salt = SaltString::generate(&mut OsRng);
//     let hashed_password = Argon2::default()
//         .hash_password(body.password.as_bytes(), &salt)
//         .map_err(|e| {
//             let error_response = serde_json::json!({
//                 "status": "fail",
//                 "message": format!("Error while hashing password: {}", e),
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//         })
//         .map(|hash| hash.to_string())?;

//     let user = sqlx::query_as!(
//         User,
//         "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
//         body.name.to_string(),
//         body.email.to_string().to_ascii_lowercase(),
//         hashed_password
//     )
//     .fetch_one(&data.db)
//     .await
//     .map_err(|e| {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Database error: {}", e),
//         });
//         (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//     })?;

//     let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
//         "user": filter_user_record(&user)
//     })});

//     Ok(Json(user_response))
// }

// pub async fn login_user_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<LoginUserSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let user = sqlx::query_as!(
//         User,
//         "SELECT * FROM users WHERE email = $1",
//         body.email.to_ascii_lowercase()
//     )
//     .fetch_optional(&data.db)
//     .await
//     .map_err(|e| {
//         let error_response = serde_json::json!({
//             "status": "error",
//             "message": format!("Database error: {}", e),
//         });
//         (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//     })?
//     .ok_or_else(|| {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": "Invalid email or password",
//         });
//         (StatusCode::BAD_REQUEST, Json(error_response))
//     })?;

//     let is_valid = match PasswordHash::new(&user.password) {
//         Ok(parsed_hash) => Argon2::default()
//             .verify_password(body.password.as_bytes(), &parsed_hash)
//             .map_or(false, |_| true),
//         Err(_) => false,
//     };

//     if !is_valid {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": "Invalid email or password"
//         });
//         return Err((StatusCode::BAD_REQUEST, Json(error_response)));
//     }

//     let now = chrono::Utc::now();
//     let iat = now.timestamp() as usize;
//     let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
//     let claims: TokenClaims = TokenClaims {
//         sub: user.id.to_string(),
//         exp,
//         iat,
//     };

//     let token = encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
//     )
//     .unwrap();

//     let cookie = Cookie::build("token", token.to_owned())
//         .path("/")
//         .max_age(time::Duration::hours(1))
//         .same_site(SameSite::Lax)
//         .http_only(true)
//         .finish();

//     let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
//     response
//         .headers_mut()
//         .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
//     Ok(response)
// }

// pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let cookie = Cookie::build("token", "")
//         .path("/")
//         .max_age(time::Duration::hours(-1))
//         .same_site(SameSite::Lax)
//         .http_only(true)
//         .finish();

//     let mut response = Response::new(json!({"status": "success"}).to_string());
//     response
//         .headers_mut()
//         .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
//     Ok(response)
// }

// pub async fn get_me_handler(
//     Extension(user): Extension<User>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let json_response = serde_json::json!({
//         "status":  "success",
//         "data": serde_json::json!({
//             "user": filter_user_record(&user)
//         })
//     });

//     Ok(Json(json_response))
// }

// fn filter_user_record(user: &User) -> FilteredUser {
//     FilteredUser {
//         id: user.id.to_string(),
//         email: user.email.to_owned(),
//         name: user.name.to_owned(),
//         photo: user.photo.to_owned(),
//         role: user.role.to_owned(),
//         verified: user.verified,
//         createdAt: user.created_at.unwrap(),
//         updatedAt: user.updated_at.unwrap(),
//     }
// }