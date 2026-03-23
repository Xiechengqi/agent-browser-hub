use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Strategy {
    Public,
    Cookie,
    Header,
    Intercept,
    Ui,
}

impl Strategy {
    pub fn requires_browser(&self) -> bool {
        !matches!(self, Strategy::Public)
    }

    pub fn requires_pre_navigation(&self) -> bool {
        matches!(self, Strategy::Cookie | Strategy::Header)
    }

    pub fn pre_navigation_url(&self, domain: &str) -> Option<String> {
        if self.requires_pre_navigation() {
            Some(format!("https://{}", domain))
        } else {
            None
        }
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Strategy::Public
    }
}
