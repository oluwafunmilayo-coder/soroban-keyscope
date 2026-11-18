pub mod collision_detector;
pub mod types;

pub use collision_detector::{AnalysisReport, CollisionDetector, KeyViolation};
pub use types::{KeyType, StorageDomain, StorageKeyDefinition};
