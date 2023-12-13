//Need the macro use text to reference schema in other files.
#[macro_use] extern crate diesel;
extern crate dotenv;

//db imports
mod schema;
mod models;

use actix_web::{App, HttpServer};
use actix_service::Service;
use actix_cors::Cors;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod views;
mod to_do;
mod state;
mod processes;
mod json_serialization;
mod jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));


    println!("Host running on 127.0.0.1:8000");
    HttpServer::new(|| {
        //Defining Cross origin Resource sharing
        // let cors = Cors::default().
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        //This shoud allow all cross site interactions to work same as above.
        //let cors = Cors::permissive();
        let app = App::new()
            //Middleware async block to check request
            .wrap_fn(|req, srv|{
                println!("/////Request info///// {:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
        }).configure(views::views_factory).wrap(cors);
        return app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

