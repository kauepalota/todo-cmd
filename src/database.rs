use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use futures::TryStreamExt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Todo {
    value: String
}

pub async fn connect() -> mongodb::error::Result<Database> {
    let options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(options)?;

    Ok(client.database("todos"))
}

pub async fn add_todo(todo: &str) -> mongodb::error::Result<()> {
    let db = connect().await?;
    let collection = db.collection::<Todo>("todos");

    let todo = Todo {
        value: todo.to_owned()
    };

    collection.insert_one(todo, None).await?;

    Ok(())
}

pub async fn get_todos() -> mongodb::error::Result<Vec<String>> {
    let db = connect().await?;
    let collection = db.collection::<Todo>("todos");

    let mut cursor = collection.find(None, None).await?;

    let mut todos = vec![];

    while let Some(todo) = cursor.try_next().await? {
        todos.push(todo.value);
    }

    Ok(todos)
}

pub async fn remove_todo(todo: Vec<String>) -> mongodb::error::Result<()> {
    let db = connect().await?;
    let collection = db.collection::<Todo>("todos");

    let filter = doc! { "value": { "$in": todo } };
    collection.delete_many(filter, None).await?;

    Ok(())
}