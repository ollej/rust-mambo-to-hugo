use regex::Regex;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use std::io::prelude::*;
use std::{fs, io};
use unidecode::unidecode;

static DIR: &str = "./content";

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

#[derive(Debug, sqlx::FromRow)]
struct Story {
    #[sqlx(rename = "sid")]
    id: i32,
    title: String,
    introtext: String,
    #[sqlx(rename = "fultext")]
    content: String,
    time: NaiveDateTime,
    author: String,
    counter: u32,
    #[sqlx(rename = "catid")]
    category_id: i32,
    hits: i32,
    archived: bool,
    newsimage: String,
    published: bool,
    image_position: String,
    ordering: i32,
    frontpage: bool,
    approved: bool,
    access: bool,
}

#[derive(Debug)]
struct HugoContent {
    category: String,
    title: String,
    author: String,
    date: NaiveDateTime,
    content: String,
    draft: bool,
}

impl HugoContent {
    fn filename(&self) -> String {
        format!(
            "{DIR}/{}/{}.md",
            sanitize(self.category.to_owned()),
            sanitize(self.title.to_owned())
        )
    }

    fn write(&self) -> Result<(), io::Error> {
        let mut file = fs::File::create(self.filename())?;
        file.write_all(self.content().as_bytes())?;
        Ok(())
    }

    fn content(&self) -> String {
        format!(
            r#"+++
Title = "{}"
Date = "{}"
Author = "{}"
Draft = {}
+++

{}"#,
            self.title,
            self.date,
            self.author,
            self.draft,
            self.content.replace("\r", ""),
        )
    }
}

fn sanitize(filename: String) -> String {
    let re = Regex::new(r"[^A-Za-z0-9_-]").unwrap();
    let str = unidecode(&filename.trim().to_lowercase()).replace(" ", "_");
    re.replace_all(&str, "").to_string()
}

fn category_name(categories: &Vec<Category>, id: i32) -> Option<String> {
    categories
        .iter()
        .find(|c| c.id == id)
        .map(|c| c.name.to_owned())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@127.0.0.1:3306/theyard")
        .await?;

    // Read mambo data
    let categories = sqlx::query_as::<_, Category>(
        "SELECT categoryid,categoryname,section,published from mos_categories",
    )
    .fetch_all(&pool)
    .await?;
    let articles = sqlx::query_as::<_, Article>(
        "SELECT artid, catid, title, content, author, date, counter, approved, archived, ordering, published from mos_articles",
    )
    .fetch_all(&pool)
    .await?;
    let stories = sqlx::query_as::<_, Story>(
        "SELECT sid, title, introtext, fultext, time, author, counter, catid, hits, archived, newsimage, published, image_position, ordering, frontpage, approved, access  from mos_stories",
    )
    .fetch_all(&pool)
    .await?;

    // Create Hugo content
    fs::create_dir_all(&DIR)?;
    fs::create_dir_all(format!("{DIR}/stories"))?;
    for category in &categories {
        let filename = format!("{}/{}", DIR, sanitize(category.name.to_owned()));
        fs::create_dir_all(filename)?;
    }
    for story in stories {
        let content = HugoContent {
            category: "Stories".to_string(),
            title: story.title,
            author: story.author,
            date: story.time,
            content: format!("{}\n{}", story.introtext, story.content),
            draft: !story.published,
        };
        content.write()?;
    }
    for article in articles {
        let content = HugoContent {
            category: category_name(&categories, article.category_id).unwrap(),
            title: article.title,
            author: article.author,
            date: article.date.and_hms_opt(0, 0, 0).unwrap(),
            content: article.content,
            draft: !article.published,
        };
        content.write()?;
    }

    Ok(())
}
