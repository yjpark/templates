#[derive(Debug, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub package_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            package_name: "template".to_string(),
        }
    }
}