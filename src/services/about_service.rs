use crate::services::prelude::*;
use crate::models::about::{About, Certificate, Contact, Education, ProgLanguage, SpokenLanguage, TechStack};

pub async fn get_all_about() -> Result<Vec<About>, APIError> {
    let db = get_db();
    let result= db
        .select("about")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn get_about(id: &str) -> Result<About, APIError> {
    let db = get_db();
     let result = db
        .select(RecordId::from(("about", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    result.ok_or(APIError::NotFound)
}

pub async fn create_about(data: About) -> Result<About, APIError> {
    let db = get_db();

    let created= db
        .create("about")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_about(id: &str, data: About) -> Result<About, APIError> {
    let db = get_db();
    let updated: Option<About> = db
        .update(RecordId::from(("about", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_about(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<About> = db.delete(RecordId::from(("about", id)))
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

pub async fn get_education(id: &str) -> Result<Education, APIError> {
    let db = get_db();
    let result: Option<Education> = db
        .select(RecordId::from(("education", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    result.ok_or(APIError::NotFound)
}

pub async fn create_education(data: Education) -> Result<Education, APIError> {
    let db = get_db();
    let created = db
        .create("education")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    created.ok_or(APIError::InternalServerError)
}

pub async fn update_education(id: &str, data: Education) -> Result<Education, APIError> {
    let db = get_db();
    let updated: Option<Education> = db
        .update(RecordId::from(("education", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_education(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<Education> = db.delete(RecordId::from(("education", id)))
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

pub async fn get_certificate(id: &str) -> Result<Certificate, APIError> {
    let db = get_db();
    let result: Option<Certificate> = db
        .select(RecordId::from(("certificates", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    result.ok_or(APIError::NotFound)
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

pub async fn update_certificate(id: &str, data: Certificate) -> Result<Certificate, APIError> {
    let db = get_db();
    let updated: Option<Certificate> = db
        .update(RecordId::from(("certificates", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_certificate(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<Certificate> = db.delete(RecordId::from(("certificates", id)))
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

pub async fn update_programming_language(id: &str, data: ProgLanguage) -> Result<ProgLanguage, APIError> {
    let db = get_db();
    let updated: Option<ProgLanguage> = db
        .update(RecordId::from(("programming_languages", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_programming_language(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<ProgLanguage> = db.delete(RecordId::from(("programming_languages", id)))
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

pub async fn update_spoken_language(id: &str, data: SpokenLanguage) -> Result<SpokenLanguage, APIError> {
    let db = get_db();
    let updated: Option<SpokenLanguage> = db
        .update(RecordId::from(("spoken_languages", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_spoken_language(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<SpokenLanguage> = db.delete(RecordId::from(("spoken_languages", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}

pub async fn get_tech_stacks() -> Result<Vec<TechStack>, APIError> {
    let db = get_db();
    let result = db
        .select("tech_stacks")
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(result)
}

pub async fn create_tech_stack(data: TechStack) -> Result<TechStack, APIError> {
    let db = get_db();
    let created = db
        .create("tech_stacks")
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;

    created.ok_or(APIError::InternalServerError)
}

pub async fn update_tech_stack(id: &str, data: TechStack) -> Result<TechStack, APIError> {
    let db = get_db();
    let updated: Option<TechStack> = db
        .update(RecordId::from(("tech_stacks", id)))
        .content(data)
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    updated.ok_or(APIError::NotFound)
}

pub async fn delete_tech_stack(id: &str) -> Result<(), APIError> {
    let db = get_db();
    let _: Option<TechStack> = db.delete(RecordId::from(("tech_stacks", id)))
        .await
        .map_err(|e| APIError::Database(e.to_string()))?;
    Ok(())
}
