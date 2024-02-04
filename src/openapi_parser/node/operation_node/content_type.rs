#[derive(Debug, Clone)]
pub enum ContentType {
    ApplicationJson,
    ApplicationXWwwFormUrlencoded,
    MultipartFormData,
    TextPlain,
}

impl ContentType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "application/json" => ContentType::ApplicationJson,
            "application/x-www-form-urlencoded" => ContentType::ApplicationXWwwFormUrlencoded,
            "multipart/form-data" => ContentType::MultipartFormData,
            "text/plain" => ContentType::TextPlain,
            _ => panic!("[RequestBodyType::from_str] invalid request body type {s}"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            ContentType::ApplicationJson => "application/json".to_string(),
            ContentType::ApplicationXWwwFormUrlencoded => {
                "application/x-www-form-urlencoded".to_string()
            }
            ContentType::MultipartFormData => "multipart/form-data".to_string(),
            ContentType::TextPlain => "text/plain".to_string(),
        }
    }
}
