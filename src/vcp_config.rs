use serde::de::{self, Visitor}; // Importing necessary traits from the serde::de module for custom deserialization.
use serde::{Deserialize, Deserializer}; // Deserialize trait for implementing the deserialization and Deserializer for access to the deserializer.
use std::fmt; // Formatting traits for output.
use url::Url; // Url type from the `url` crate used for parsing URLs.

/// Custom `Visitor` for deserializing URLs.
/// `Visitor` pattern is used here to convert strings to `Url` types during deserialization.
pub struct UrlVisitor;

impl<'de> Visitor<'de> for UrlVisitor {
    type Value = Url;

    /// Provides a custom description for the expected element when deserialization fails.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid URL string")
    }

    /// Attempts to convert a string to a `Url` type.
    /// This function is called by Serde when it encounters a string during deserialization.
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error, // E represents the error type that can be returned during deserialization.
    {
        // Parse the string as a `Url`, mapping any errors using Serde's custom error handler to provide better error context.
        Url::parse(value).map_err(de::Error::custom)
    }
}

/// Custom deserialization function to use with Serde for fields expecting a `Url` type.
/// This function is used as an attribute in struct definitions to specify how to deserialize `Url` fields.
pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>, // The Deserializer trait provides methods to deserialize data.
{
    // Delegate the deserialization to the `UrlVisitor`, which implements the conversion logic.
    deserializer.deserialize_str(UrlVisitor)
}

/// Configuration structure to hold settings that can be loaded from a configuration file.
#[derive(Deserialize)]
pub struct Config {
    /// The `server_url` field uses custom deserialization to ensure the URL is correctly parsed.
    /// The `deserialize_with` attribute points to the `deserialize_url` function to handle this field.
    #[serde(deserialize_with = "deserialize_url")]
    pub server_url: Url,
}
