use std::env;

pub const DEFAULT_BRAND_NAME: &str = "Codex Native";
pub const DEFAULT_AGENT_NAME: &str = "Codex Native Agent";
pub const DEFAULT_DESCRIPTION: &str = "Native AI coding desktop";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppIdentity {
    pub brand_name: String,
    pub agent_name: String,
    pub description: String,
}

impl AppIdentity {
    pub fn from_environment() -> Self {
        let brand_name = configured("APP_BRAND_NAME").unwrap_or_else(|| DEFAULT_BRAND_NAME.into());
        let agent_name =
            configured("APP_AGENT_NAME").unwrap_or_else(|| format!("{brand_name} Agent"));
        let description =
            configured("APP_DESCRIPTION").unwrap_or_else(|| DEFAULT_DESCRIPTION.into());
        Self {
            brand_name,
            agent_name,
            description,
        }
    }

    pub fn template(&self, value: &str) -> String {
        value
            .replace("{brand}", &self.brand_name)
            .replace("{agent}", &self.agent_name)
            .replace("{description}", &self.description)
    }
}

fn configured(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::{AppIdentity, DEFAULT_BRAND_NAME};

    #[test]
    fn defaults_are_stable() {
        let identity = AppIdentity {
            brand_name: DEFAULT_BRAND_NAME.into(),
            agent_name: "Codex Native Agent".into(),
            description: "Native AI coding desktop".into(),
        };
        assert_eq!(
            identity.template("{brand} / {agent} / {description}"),
            "Codex Native / Codex Native Agent / Native AI coding desktop"
        );
    }

    #[test]
    fn empty_values_are_not_used_by_templates() {
        let identity = AppIdentity {
            brand_name: "Example".into(),
            agent_name: "Example Agent".into(),
            description: "Desktop app".into(),
        };
        assert_eq!(identity.template(" {brand} "), " Example ");
    }
}
