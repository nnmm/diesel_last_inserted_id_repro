use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncConnection, AsyncMysqlConnection, RunQueryDsl};
mod schema;

use schema::*;

async fn create_user_with_last_insert_id(
    conn: &mut AsyncMysqlConnection,
    name: &str,
) -> Result<i32, diesel::result::Error> {
    // A transaction is needed for LAST_INSERT_ID() to really return the id of the inserted object IIUC
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            diesel::insert_into(users::table)
                .values(users::name.eq(name))
                .execute(conn)
                .await?;

            // Get the ID using LAST_INSERT_ID() - this creates a new prepared statement each time!
            let user_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result(conn)
            .await?;

            Ok(user_id)
        })
    })
    .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;

    let conn_manager = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);
    let pool = Pool::builder(conn_manager)
        .max_size(10)
        .build()?;

    let num_iterations = 30000;

    println!("Running {num_iterations} INSERT operations with LAST_INSERT_ID()...",);

    for i in 0..num_iterations {
        let mut conn = pool.get().await?;

        let user_name = format!("user_{i}");
        let _user_id = create_user_with_last_insert_id(&mut conn, &user_name).await?;
    }

    println!("Done.");

    Ok(())
}
