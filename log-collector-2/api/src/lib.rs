use serde_derive::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

pub mod logs {
    pub mod get {
        pub type Query = crate::DateTimeRange;
        pub struct Response(pub Vec<crate::Log>);
    }

    pub mod post {}
}

pub mod csv {
    pub mod get {}

    pub mod post {}
}
