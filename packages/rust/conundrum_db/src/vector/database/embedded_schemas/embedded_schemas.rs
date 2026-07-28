use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src/vector/database/embedded_schemas/"]
pub struct EmbeddedSchemas;

impl EmbeddedSchemas {
    fn read_string(fp: &str) -> Option<String> {
        if let Some(res) = EmbeddedSchemas::get(fp)
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            return Some(body.to_string());
        } else {
            return None;
        }
    }

    pub fn current_schema() -> String {
        Self::read_string("pre_release.sql").expect("Must unwrap current embedded schema.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwraps_current_schema() {
        let schema = EmbeddedSchemas::current_schema();
        assert!(schema.len() > 0, "Current schema is not empty.");
    }
}
