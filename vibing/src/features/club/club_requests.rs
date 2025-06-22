use reqwest::Method;

use crate::{error::Result, features::club::response::ClubResponse, requests::fetch_backend};

pub async fn get_club_info(club_id: u32, verbose: bool) -> Result<()> {
    let formatted_path = format!("/api/v1/club/{club_id}").to_owned();
    let club_response =
        fetch_backend::<(), ClubResponse>(formatted_path, Method::GET, None, verbose).await?;
    println!("{club_response:#?}");
    Ok(())
}
