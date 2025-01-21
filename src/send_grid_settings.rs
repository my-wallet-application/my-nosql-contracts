use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("send-grid-settings")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SendGridSettingsMyNoSqlEntity {
    pub send_grid_api_key: String,
    pub email_from: String,
    pub email_from_name: Option<String>,
    pub email_cc: Option<String>,
    pub email_bcc: Option<String>,
    pub mail_logo_url: Option<String>,
    pub mail_success_picture_url: Option<String>,
    pub mail_fail_picture_url: Option<String>,
    pub policy_url: String,
    pub terms_url: String,
    pub about_url: String,
    pub support_url: String,
    pub home_page_url: String,
}

impl SendGridSettingsMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "sg"
    }

    pub fn generate_row_key(src_id: &'static str) -> &'static str {
        src_id
    }
}
