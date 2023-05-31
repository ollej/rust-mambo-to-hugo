use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::NaiveDate;

#[derive(Debug, sqlx::FromRow)]
struct Category {
    #[sqlx(rename = "categoryid")]
    id: i32,
    #[sqlx(rename = "categoryname")]
    name: String,
    section: String,
    published: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct Article {
    #[sqlx(rename = "artid")]
    id: i32,
    #[sqlx(rename = "catid")]
    category_id: i32,
    title: String,
    content: String,
    author: String,
    date: NaiveDate,
    counter: i32,
    approved: bool,
    archived: bool,
    ordering: i32,
    published: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@127.0.0.1:3306/theyard")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let categories = sqlx::query_as::<_, Category>(
        "SELECT categoryid,categoryname,section,published from mos_categories",
    )
    .fetch_all(&pool)
    .await?;

    for category in categories {
        // map the row into a user-defined domain type
        println!("Category: {:?}", category);
    }

    let articles = sqlx::query_as::<_, Article>(
        "SELECT artid, catid, title, content, author, date, counter, approved, archived, ordering, published from mos_articles",
    )
    .fetch_all(&pool)
    .await?;

    for article in articles {
        // map the row into a user-defined domain type
        println!("Article: {:?}", article);
    }

    Ok(())
}
