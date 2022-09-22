use std::borrow::Cow;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Crypto<'s> {
    /// The id of the crypto in the akasha database
    #[serde(skip_serializing_if = "Option::is_none", rename = "internal_id")]
    id: Option<String>,
    /// the name service where the information was collected.
    #[serde(skip_serializing)]
    service_name: Option<Cow<'s, str>>,
    /// ID of the asset in the platform.
    #[serde(rename = "id")]
    platform_defined_id: String,
    /// The name of the asset.
    name: String,
    /// The symbol of the asset.
    symbol: String,
}

impl Crypto<'_> {
    pub fn new(
        platform_defined_id: String,
        name: String,
        symbol: String,
    ) -> Self {
        Crypto {
            id: None,
            service_name: None,
            platform_defined_id,
            name,
            symbol
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn platform(&self) -> Option<&str> {
       match self.service_name.as_ref(){
              Some(s) => Some(s.as_ref()),
              None => None
       }
    }
    pub fn platform_defined_id(&self) -> &String {
        &self.platform_defined_id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_service_name(&mut self, name:String){
        self.service_name = Some(Cow::from(name));
    }
}



#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CryptImageURL {
    small: Option<String>,
    large: Option<String>,
    thumb: Option<String>,
}

impl CryptImageURL {
    pub fn new(small: Option<String>, large: Option<String>, thumb: Option<String>) -> Self {
        Self { small, large, thumb }
    }
    pub fn small(&self) -> &Option<String> {
        &self.small
    }
    pub fn large(&self) -> &Option<String> {
        &self.large
    }
    pub fn thumb(&self) -> &Option<String> {
        &self.thumb
    }
}