mod logger;

type StdErr = Box<dyn std::error::Error>;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}


fn main() -> Result<(), StdErr> {
    dotenv::dotenv();
    logger::init();
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());
    Ok(())
}