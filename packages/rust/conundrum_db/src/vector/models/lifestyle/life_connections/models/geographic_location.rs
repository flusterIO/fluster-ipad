use crate::vector::models::lifestyle::life_connections::models::physical_street_address::PhysicalStreetAddress;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct GeographicLocation {
    pub address: Option<PhysicalStreetAddress>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
