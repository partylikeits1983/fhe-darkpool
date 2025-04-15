use serde::{Deserialize, Serialize};
use tfhe::named::Named;
use tfhe::{Unversionize, Versionize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: u32,
    pub asset_a: u32,
    pub asset_b: u32,
    pub price: u32,
    pub a_for_b: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Orders {
    pub order: Vec<Order>,
}

pub fn safe_serialize_item<T>(item: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    T: serde::Serialize + Versionize + Named,
{
    let mut buf = Vec::new();
    // up to 1 MB
    tfhe::safe_serialization::safe_serialize(item, &mut buf, 1 << 20)?;
    Ok(buf)
}

pub fn safe_deserialize_item<T>(data: &[u8]) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + Unversionize + Named,
{
    use std::io::Cursor;
    let cursor = Cursor::new(data);
    let item = tfhe::safe_serialization::safe_deserialize(cursor, 1 << 20)?;
    Ok(item)
}
