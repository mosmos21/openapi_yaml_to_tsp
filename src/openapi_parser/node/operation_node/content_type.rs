use std::fmt::Display;

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
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_type = match self {
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationXWwwFormUrlencoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::TextPlain => "text/plain",
        };
        write!(f, "{}", content_type)
    }
}
