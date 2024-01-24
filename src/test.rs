use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn ping() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({"result":"pong"})))
}

fn fib_recursive(n: u32) -> u128 {
    match n {
        1 | 2 => 1,
        3 => 2,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

fn fib_iterative(n: u32) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut c;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    a
}
pub async fn fibr(
    Path(n): Path<u32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({"n": n, "fibonacci": fib_recursive(n)})))
}
pub async fn fibi(
    Path(n): Path<u32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({"n": n, "fibonacci": fib_iterative(n)})))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibi() {
        let n = 100;
        let result = fib_iterative(n);
        assert_eq!(result, 354_224_848_179_261_915_075);
        let j: serde_json::Value = json!({"n": n, "fibonacci": fib_iterative(n)});
        println!("{j}");
        assert!(!j.is_u64());
    }
}
