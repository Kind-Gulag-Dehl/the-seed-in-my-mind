use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub height: u64,
    pub created_at: String,
    pub state_root_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaSummary {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaDetail {
    pub id: Uuid,
    pub title: String,
    pub connections: Vec<IdeaEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaEdge {
    pub to: Uuid,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neighborhood {
    pub center: Uuid,
    pub nodes: Vec<IdeaSummary>,
    pub edges: Vec<IdeaEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub query: String,
    pub results: Vec<IdeaSummary>,
}
