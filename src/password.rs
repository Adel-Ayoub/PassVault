use aes_gcm::{aes::cipher::ArrayLength, Nonce};
use base64::prelude::*;

#[derive(Default)]
pub struct Password {
    pub value: String,
    pub name: String,
    pub description: String,
    pub date: String,
    pub is_enc: bool,
}

impl Password {
    // Generate a new password
    pub fn new(name: String, description: String, date: String) -> Self {
        Password {
            value: String::default(),
            name,
            description,
            date,
            is_enc: false,
        }
    }
    pub fn set_value<T: ArrayLength<u8>>(&mut self, data: Vec<u8>, nonce: Nonce<T>) {
        let mut l_data: Vec<u8> = Vec::new();
        l_data.extend_from_slice(nonce.as_slice());
        l_data.extend(data);
        self.value = BASE64_STANDARD.encode(l_data);
    }
    pub fn is_enc(&self) -> bool {
        self.is_enc
    }
}
