use crate::{error::APIError, models::about::Education};
use crate::models::about::{About, Certificate, Contact, ProgLanguage, SpokenLanguage};
use crate::services::about_service;
use actix_web::{HttpResponse, Result, delete, get, post, put, web};
use uuid::Uuid;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_about)
        .service(get_about)
        .service(get_all_education)
        .service(get_education)
        .service(get_contact)
        .service(get_all_certificates)
        .service(get_certificate)
        .service(get_programming_languages)
        .service(get_spoken_languages)
        .service(get_tech_stacks);
}

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_about)
        .service(put_about)
        .service(delete_about)
        .service(create_education)
        .service(put_education)
        .service(delete_education)
        .service(create_contact)
        .service(put_contact)
        .service(delete_contact)
        .service(create_certificate)
        .service(put_certificate)
        .service(delete_certificate)
        .service(create_programming_language)
        .service(put_programming_language)
        .service(delete_programming_language)
        .service(create_spoken_language)
        .service(put_spoken_language)
        .service(delete_spoken_language)
        .service(create_tech_stack)
        .service(put_tech_stack)
        .service(delete_tech_stack);
}


#[get("/about/all")]
async fn get_all_about() -> Result<HttpResponse, APIError> {
    let abouts = about_service::get_all_about().await?;
    if abouts.is_empty() {
        return Err(APIError::NotFound);
    }
    Ok(HttpResponse::Ok().json(abouts))
}

#[get("/about/{id}")]
async fn get_about(id: web::Path<String>) -> Result<HttpResponse, APIError> {
    let about = about_service::get_about(&id).await?;
    Ok(HttpResponse::Ok().json(about))
}

#[post("/about/{id}")]
async fn create_about(id: web::Path<String>, payload: web::Json<About>) -> Result<HttpResponse, APIError> {
    let about = about_service::create_about(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(about))
}

#[put("/about/{id}")]
async fn put_about(id: web::Path<String>, payload: web::Json<About>) -> Result<HttpResponse, APIError> {
    let updated_about = about_service::update_about( &id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_about))
}

#[delete("/about/{id}")]
async fn delete_about(id: web::Path<String>) -> Result<HttpResponse, APIError> {
    about_service::delete_about(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/education/all")]
async fn get_all_education() -> Result<HttpResponse, APIError> {
    let educations = about_service::get_all_education().await?;
    Ok(HttpResponse::Ok().json(educations))
}

#[post("/education")]
async fn create_education(payload: web::Json<Education>) -> Result<HttpResponse, APIError> {
    let education = about_service::create_education(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(education))
}
#[get("/education/{id}")]
async fn get_education(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    let education = about_service::get_education(&id).await?;
    Ok(HttpResponse::Ok().json(education))
}

#[put("/education/{id}")]
async fn put_education(
    id: web::Path<Uuid>,
    payload: web::Json<Education>,
) -> Result<HttpResponse, APIError> {
    let updated_education = about_service::update_education(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_education))
}

#[delete("/education/{id}")]
async fn delete_education(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    about_service::delete_education(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/contact")]
async fn get_contact() -> Result<HttpResponse, APIError> {
    let contact = about_service::get_contact().await?;
    Ok(HttpResponse::Ok().json(contact))
}

#[post("/contact")]
async fn create_contact(payload: web::Json<Contact>) -> Result<HttpResponse, APIError> {
    let contact = about_service::create_contact(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(contact))
}

#[put("/contact")]
async fn put_contact(payload: web::Json<Contact>) -> Result<HttpResponse, APIError> {
    let updated_contact = about_service::update_contact(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_contact))
}

#[delete("/contact")]
async fn delete_contact() -> Result<HttpResponse, APIError> {
    about_service::delete_contact().await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/certificates/all")]
async fn get_all_certificates() -> Result<HttpResponse, APIError> {
    let certificates = about_service::get_all_certificates().await?;
    Ok(HttpResponse::Ok().json(certificates))
}

#[get("/certificates/{id}")]
async fn get_certificate(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    let certificate = about_service::get_certificate(&id).await?;
    Ok(HttpResponse::Ok().json(certificate))
}

#[post("/certificates")]
async fn create_certificate(payload: web::Json<Certificate>) -> Result<HttpResponse, APIError> {
    let certificate = about_service::create_certificate(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(certificate))
}

#[put("/certificates/{id}")]
async fn put_certificate(
    id: web::Path<Uuid>,
    payload: web::Json<Certificate>,
) -> Result<HttpResponse, APIError> {
    let updated_certificate = about_service::update_certificate(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_certificate))
}

#[delete("/certificates/{id}")]
async fn delete_certificate(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    about_service::delete_certificate(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/programming-languages")]
async fn get_programming_languages() -> Result<HttpResponse, APIError> {
    let languages = about_service::get_programming_languages().await?;
    Ok(HttpResponse::Ok().json(languages))
}

#[post("/programming-languages")]
async fn create_programming_language(payload: web::Json<ProgLanguage>) -> Result<HttpResponse, APIError> {
    let language = about_service::create_programming_language(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(language))
}

#[put("/programming-languages/{id}")]
async fn put_programming_language(
    id: web::Path<Uuid>,
    payload: web::Json<ProgLanguage>,
) -> Result<HttpResponse, APIError> {
    let updated_language = about_service::update_programming_language(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_language))
}

#[delete("/programming-languages/{id}")]
async fn delete_programming_language(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    about_service::delete_programming_language(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/spoken-languages")]
async fn get_spoken_languages() -> Result<HttpResponse, APIError> {
    let languages = about_service::get_spoken_languages().await?;
    Ok(HttpResponse::Ok().json(languages))
}

#[post("/spoken-languages")]
async fn create_spoken_language(payload: web::Json<SpokenLanguage>) -> Result<HttpResponse, APIError> {
    let language = about_service::create_spoken_language(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(language))
}

#[put("/spoken-languages/{id}")]
async fn put_spoken_language(
    id: web::Path<Uuid>,
    payload: web::Json<SpokenLanguage>,
) -> Result<HttpResponse, APIError> {
    let updated_language = about_service::update_spoken_language(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_language))
}

#[delete("/spoken-languages/{id}")]
async fn delete_spoken_language(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    about_service::delete_spoken_language(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/tech-stacks")]
async fn get_tech_stacks() -> Result<HttpResponse, APIError> {
    let tech_stacks = about_service::get_tech_stacks().await?;
    Ok(HttpResponse::Ok().json(tech_stacks))
}

#[post("/tech-stacks")]
async fn create_tech_stack(payload: web::Json<String>) -> Result<HttpResponse, APIError> {
    let tech_stack = about_service::create_tech_stack(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(tech_stack))
}

#[put("/tech-stacks/{id}")]
async fn put_tech_stack(
    id: web::Path<Uuid>,
    payload: web::Json<String>,
) -> Result<HttpResponse, APIError> {
    let updated_tech_stack = about_service::update_tech_stack(&id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_tech_stack))
}

#[delete("/tech-stacks/{id}")]
async fn delete_tech_stack(id: web::Path<Uuid>) -> Result<HttpResponse, APIError> {
    about_service::delete_tech_stack(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}
