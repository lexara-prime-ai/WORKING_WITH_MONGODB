#![allow(unused)]

/*
    Project Title: Working with MongoDB
    Author: Irfan M. Ghat
    Year: 2024
*/

mod models;

use bson::oid::ObjectId;
use bson::Document;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use std::env;
use std::error::Error;
use tokio::main;

macro_rules! logger {
    ($val: expr) => {
        println!("-> {:#?}", $val);
    };
}

#[derive(Debug, Deserialize, Serialize)]
struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,pub year: i32,
    plot: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    released: chrono::DateTime<Utc>
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn_str = env::var("MONGODB_URI").expect("[MONGODB_URI] must be set.");
    let options = ClientOptions::parse(conn_str).await?;
    let client = Client::with_options(options)?;
    let movies = client
        .database("sample_mflix")
        .collection::<Document>("movies");

    // -> Sample document.
    // let doc = doc! {
    //     "title": "Parasite",
    //     "year": 2024,
    //     "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
    //     "released": bson::DateTime::builder()
    //     .year(2024)
    //     .month(2)
    //     .day(12)
    //     .build()?
    // };

    // -> Insert a document.
    // let result = movies.insert_one(doc.clone()).await?;
    // logger!(result);

    // -> Retrieve a document.
    let movie: Document = movies
        .find_one(doc! {
            "title": "Parasite"
        })
        .await?
        .expect("Document not found.");

    // logger!(movie);

    // -> Update a document.
    // let result = movies
    //     .update_one(
    //         doc! {
    //             "_id": movie.get("_id")
    //         },
    //         doc! {
    //             "$set": {"year": 2019}
    //         },
    //     )
    //     .await?;

    // logger!(result.modified_count);

    // -> Delete a document.
    // let result = movies
    //     .delete_one(doc! {
    //         "_id": movie.get("_id")
    //     })
    //     .await?;

    // logger!(result.deleted_count);

    Ok(())
}
