use crate::type_spec::node::decorators::{LibInfo, TypeSpecDecorator};
use crate::type_spec::node::*;
use std::collections::HashMap;
use std::fmt::Display;

fn hash_map_to_string(map: &HashMap<String, String>) -> String {
    let properties = map
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join(",\n");

    format!("{{\n{}\n}}", properties)
}

#[derive(Debug)]
pub struct AdditionalInfoNode {
    pub contact: Option<ContactNode>,
    pub license: Option<LicenseNode>,
    pub terms_of_service: Option<String>,
}

#[derive(Debug)]
pub struct ContactNode {
    pub email: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct LicenseNode {
    name: String,
    url: Option<String>,
}

impl Display for AdditionalInfoNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = HashMap::new();

        if let Some(contact) = &self.contact {
            map.insert("contact".to_string(), contact.to_string());
        }
        if let Some(license) = &self.license {
            map.insert("license".to_string(), license.to_string());
        }
        if let Some(terms_of_service) = &self.terms_of_service {
            map.insert(
                "termsOfService".to_string(),
                string_literal(&terms_of_service),
            );
        }

        write!(f, "@info({})", hash_map_to_string(&map))
    }
}

impl Display for ContactNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = HashMap::new();

        if let Some(email) = &self.email {
            map.insert("email".to_string(), string_literal(&email));
        }
        if let Some(name) = &self.name {
            map.insert("name".to_string(), string_literal(&name));
        }
        if let Some(url) = &self.url {
            map.insert("url".to_string(), string_literal(&url));
        }

        write!(f, "{}", hash_map_to_string(&map))
    }
}

impl Display for LicenseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = HashMap::new();
        map.insert("name".to_string(), string_literal(&self.name));
        if let Some(url) = &self.url {
            map.insert("url".to_string(), string_literal(url));
        }

        write!(f, "{}", hash_map_to_string(&map))
    }
}

impl LibInfo for AdditionalInfoNode {
    fn get_lib_name(&self) -> Option<&'static str> {
        Some("@typespec/openapi")
    }
    fn get_namespace(&self) -> Option<&'static str> {
        Some("TypeSpec.OpenAPI")
    }
}

impl TypeSpecDecorator for AdditionalInfoNode {}

impl NameSpaceDecorator for AdditionalInfoNode {}
