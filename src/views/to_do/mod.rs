mod create;
mod get;
mod edit;
mod delete;

use actix_web::web::{ServiceConfig, post, scope, get};

//define views and urls
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
        // Creates line in json for todo task
            .route("create/{title}", post().to(create::create))
        //Gets json for tasks
            .route("get", get().to(get::get))
        //Edits json with tasks, should work with out of sync frontends 
            .route("edit", post().to(edit::edit)) 
            .route("delete", post().to(delete::delete))
    );
}

