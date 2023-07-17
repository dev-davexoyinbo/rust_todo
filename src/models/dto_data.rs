use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseDTO<T> {
    message: String,
    data: T,
}

impl<T> ResponseDTO<T> {
    pub fn new(data: T) -> Self {
        return ResponseDTO {
            data,
            message: String::from("Success"),
        };
    }

    pub fn message(mut self, message_str: &str) -> Self {
        self.message = String::from(message_str);
        return self;
    }
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ResponseDTO<T> {
//     data: T,
// }

// impl<T> ResponseDTO<T> {
//     pub fn new(data: T) -> Self {
//         return ResponseDTO { data };
//     }
// }
