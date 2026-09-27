use crate::PluralKit;

pub struct PluralKitBuilder {
    user_agent: String,
    base_url: String,
}

impl PluralKitBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<PluralKit, crate::Error> {
        PluralKit::with_user_agent_and_base_url(&self.user_agent, &self.base_url)
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

impl Default for PluralKitBuilder {
    fn default() -> Self {
        Self {
            user_agent: format!("pluralkit-rs/{}", env!("CARGO_PKG_VERSION")),
            base_url: String::from("https://api.pluralkit.me/v2"),
        }
    }
}
