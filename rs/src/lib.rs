// use sqlx::{PgPool, Row};



// #[sqlx::test]
// async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
//     let mut conn = pool.acquire().await?;

//     let foo = sqlx::query("SELECT * FROM pyme")
//         .fetch_one(&mut conn)
//         .await?;

//     assert_eq!(foo.get::<String, _>("customer"), "ALICIA");
    
//     Ok(())
// }