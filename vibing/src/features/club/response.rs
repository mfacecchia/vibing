use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClubResponse {
    pub created_by: String,
    pub modified_by: String,
    pub deleted: bool,
    pub id: u32,
    pub club_name: String,
    pub vat_code: String,
    pub address: String,
    pub category: String,
    pub opening_time: String,
    pub closing_time: String,
    pub phone_number: String,
    pub image: Option<String>,
}
