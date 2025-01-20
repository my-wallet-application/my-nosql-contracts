use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("send-grid-mapping")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridMappingMyNoSqlEntity {
    //pub templates: HashMap<String, HashMap<String, String>>,
    pub template_id: Option<String>,
}

impl SendGridMappingMyNoSqlEntity {
    pub fn generate_partition_key(lang_id: &str) -> &str {
        lang_id
    }

    pub fn generate_row_key(email_type: &EmailTypeMyNoSql) -> &str {
        email_type.as_str()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum EmailTypeMyNoSql {
    WelcomeEmail = 1,
    RecoveryPasswordCode = 2,
    AuthorizationCode = 3,
}

impl EmailTypeMyNoSql {
    pub fn get_all() -> Vec<EmailTypeMyNoSql> {
        let enums = vec![
            Self::WelcomeEmail,    
            Self::RecoveryPasswordCode,
            Self::AuthorizationCode,
            ];
        return enums;
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::WelcomeEmail => "WelcomeEmail",
            Self::RecoveryPasswordCode => "RecoveryPasswordCode",
            Self::AuthorizationCode => "AuthorizationCode",
        }
    }
}

impl ToString for EmailTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for EmailTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            1 => EmailTypeMyNoSql::WelcomeEmail,
            2 => EmailTypeMyNoSql::RecoveryPasswordCode,
            3 => EmailTypeMyNoSql::AuthorizationCode,
            _ => panic!(
                "Invalid value '{}' for EmailTypeMyNoSql",
                value,
            ),
        }
    }
}
