use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

type Repositry = web::Data<Box<dyn FilmRepository>>;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all))
            .route("/{film_id}", web::get().to(get))
            .route("", web::post().to(post))
            .route("", web::put().to(put))
            .route("/{film_id}", web::delete().to(delete)),
    );
}

async fn get_all(repo: Repositry) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}

async fn get(film_id: web::Path<Uuid>, repo: Repositry) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body(format!("Not found")),
    }
}

async fn post(create_film: web::Json<CreateFilm>, repo: Repositry) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}

async fn put(film: web::Json<Film>, repo: Repositry) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}

async fn delete(film_id: web::Path<Uuid>, repo: Repositry) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film_id) => HttpResponse::Ok().json(film_id),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}
