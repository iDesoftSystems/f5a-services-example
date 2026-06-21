use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppEnvironment {
    #[default]
    Dev,
    Prod,
}

impl AppEnvironment {
    pub fn is_swagger_ui_enabled(&self) -> bool {
        matches!(self, Self::Dev)
    }
}

impl FromStr for AppEnvironment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            _ => Err(format!(
                "invalid APP_ENVIRONMENT: {s}, expected 'dev' or 'prod'"
            )),
        }
    }
}
