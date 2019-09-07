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

pub mod csv {
    pub mod get {
        pub type Query = crate::DateTimeRange;
    }

    pub mod post {
        use serde_derive::*;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub usize);
    }
}

pub mod logs {
    pub mod get {
        use serde_derive::*;

        pub type Query = crate::DateTimeRange;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub Vec<crate::Log>);
    }

    pub mod post {
        use serde_derive::*;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
        }
    }
}
