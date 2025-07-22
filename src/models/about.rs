use serde::{Deserialize, Serialize, Serializer, Deserializer};
use surrealdb::sql::Thing;

#[derive(Debug, Clone)]
pub struct RecordIdReturn(pub String);

impl<'de> Deserialize<'de> for RecordIdReturn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        Ok(RecordIdReturn(thing.id.to_string()))
    }
}

impl Serialize for RecordIdReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct About {
    pub id: Option<RecordIdReturn>,
    pub name: String,
    pub headline: String,
    pub description: String,
    pub location: Location,
    pub interests: Vec<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EducationType {
    School,
    University,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Education {
    pub id: Option<RecordIdReturn>,
    pub name: String,
    pub r#type: EducationType,
    pub degree: Option<String>,
    pub class: Option<String>,
    pub specialization: Option<String>,
    pub location: Location,
    pub year: YearRange,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YearRange {
    pub from: i32,
    pub to: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub professional_email: Option<String>,
    pub personal_email: String,
    pub github: String,
    pub linkedin: String,
    pub twitter: String,
    pub instagram: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    pub id: Option<RecordIdReturn>,
    pub title: String,
    pub issuer: Vec<String>,
    pub url: Option<String>,
    pub year: i32,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProgrammingLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgLanguage {
    pub id: Option<RecordIdReturn>,
    pub name: String,
    pub level: ProgrammingLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Fluent,
    Native,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpokenLanguage {
    pub id: Option<RecordIdReturn>,
    pub name: String,
    pub proficiency: ProficiencyLevel,
}
