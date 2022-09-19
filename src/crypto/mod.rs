pub mod service;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Crypto {
    /// the name where the information was collected.
    platform: Option<String>,
    /// ID of the asset in the platform.
    id: String,
    /// The name of the asset.
    name: String,
    /// The symbol of the asset.
    symbol: String,
    /// The Image URL for the asset.
    image:Option<CryptImageURL>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CryptoDevInfo{
    image:CryptImageURL
}

#[derive(Debug,Deserialize, Clone)]
#[allow(dead_code)]
pub struct CryptImageURL {
    pub small: String,
    pub large: String,
    pub thumb: String,
}
