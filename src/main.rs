/*
    Project Title: Working with MongoDB
    Author: Irfan M. Ghat
    Year: 2024
*/

use mongodb::{options::ClientOptions, Client};
use std::env;
use std::error::Error;
use tokio::main;

macro_rules! logger {
    ($val: expr) => {
        println!("-> {:#?}", $val);
    };
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn_str = env::var("MONGODB_URI").expect("[MONGODB_URI] must be set.");
    let options = ClientOptions::parse(conn_str).await?;
    let client = Client::with_options(options)?;

    logger!("Listing databases...");

    for val in client.list_database_names().await? {
        logger!(val);
    }

    Ok(())
}
