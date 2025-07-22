use uuid::Uuid;

use crate::services::prelude::*;
use crate::models::about::{About, Certificate, Contact, Education, ProgLanguage, SpokenLanguage};

pub async fn get_all_about() -> Result<Vec<About>, APIError> {
    let db = get_db();
    let result= db
        .select("about")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn get_about(title: &str) -> Result<About, APIError> {
    let db = get_db();
    let result = db
        .select(RecordId::from(("about", title)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result.ok_or(APIError::NotFound)?)
}

pub async fn create_about(id: &str, data: About) -> Result<About, APIError> {
    let db = get_db();

    let created= db
        .create(RecordId::from(("about", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_about(id: &str, data: About) -> Result<About, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("about", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_about(id: &str) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<About>>(RecordId::from(("about", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_all_education() -> Result<Vec<Education>, APIError> {
    let db = get_db();
    let result = db
        .select("education")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn get_education(id: &Uuid) -> Result<Vec<Education>, APIError> {
    let db = get_db();
    let result = db
        .select(RecordId::from(("education", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result.into_iter().collect())
}

pub async fn create_education(data: Education) -> Result<Education, APIError> {
    let db = get_db();
    let created = db
        .create(RecordId::from(("education", Uuid::new_v4())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    created.ok_or(APIError::InternalServerError)
}

pub async fn update_education(id: &Uuid, data: Education) -> Result<Education, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("education", id.clone())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_education(id: &Uuid) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<Education>>(RecordId::from(("education", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_contact() -> Result<Vec<Contact>, APIError> {
    let db = get_db();
    let result = db
        .select(RecordId::from(("contact", "default")))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result.into_iter().collect())
}

pub async fn create_contact(data: Contact) -> Result<Contact, APIError> {
    let db = get_db();
    let created = db
        .create(RecordId::from(("contact", "default")))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_contact(data: Contact) -> Result<Contact, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("contact", "default")))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_contact() -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<Contact>>(RecordId::from(("contact", "default")))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_all_certificates() -> Result<Vec<Certificate>, APIError> {
    let db = get_db();
    let result = db
        .select("certificates")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn get_certificate(id: &Uuid) -> Result<Vec<Certificate>, APIError> {
    let db = get_db();
    let result = db
        .select(RecordId::from(("certificates", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result.into_iter().collect())
}

pub async fn create_certificate(data: Certificate) -> Result<Certificate, APIError> {
    let db = get_db();
    let created = db
        .create("certificates")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_certificate(id: &Uuid, data: Certificate) -> Result<Certificate, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("certificates", id.clone())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_certificate(id: &Uuid) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<Certificate>>(RecordId::from(("certificates", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_programming_languages() -> Result<Vec<ProgLanguage>, APIError> {
    let db = get_db();
    let result = db
        .select("programming_languages")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn create_programming_language(data: ProgLanguage) -> Result<ProgLanguage, APIError> {
    let db = get_db();
    let created = db
        .create("programming_languages")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_programming_language(id: &Uuid, data: ProgLanguage) -> Result<ProgLanguage, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("programming_languages", id.clone())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_programming_language(id: &Uuid) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<ProgLanguage>>(RecordId::from(("programming_languages", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_spoken_languages() -> Result<Vec<SpokenLanguage>, APIError> {
    let db = get_db();
    let result = db
        .select("spoken_languages")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn create_spoken_language(data: SpokenLanguage) -> Result<SpokenLanguage, APIError> {
    let db = get_db();
    let created = db
        .create("spoken_languages")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_spoken_language(id: &Uuid, data: SpokenLanguage) -> Result<SpokenLanguage, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("spoken_languages", id.clone())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_spoken_language(id: &Uuid) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<SpokenLanguage>>(RecordId::from(("spoken_languages", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_tech_stacks() -> Result<Vec<String>, APIError> {
    let db = get_db();
    let result = db
        .select("tech_stacks")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn create_tech_stack(data: String) -> Result<String, APIError> {
    let db = get_db();
    let created = db
        .create("tech_stacks")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_tech_stack(id: &Uuid, data: String) -> Result<String, APIError> {
    let db = get_db();
    let updated = db
        .update(RecordId::from(("tech_stacks", id.clone())))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    updated.ok_or(APIError::InternalServerError)
}

pub async fn delete_tech_stack(id: &Uuid) -> Result<(), APIError> {
    let db = get_db();
    db.delete::<Option<String>>(RecordId::from(("tech_stacks", id.clone())))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}
