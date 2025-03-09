use super::model::*;
use super::Error;
use super::{delete, get, handle_response, patch, API_URL};
use crate::auth::OAuth;
use serde::Serialize;

/// Update specified anime in animelist
#[derive(Clone, Debug, Serialize)]
pub struct UpdateUserAnimeListStatusQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserWatchStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rewatching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_watched_episodes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_times_rewatched: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewatch_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

pub async fn update_anime_list_status(
    anime_id: u64,
    update: &UpdateUserAnimeListStatusQuery,
    auth: &OAuth,
) -> Result<UserAnimeListStatus, Error> {
    let response = patch(
        &format!("{}/anime/{}/my_list_status", API_URL, anime_id,),
        auth,
        update,
    )
    .await?;
    handle_response(&response)
}

pub async fn delete_anime_from_list(anime_id: u64, auth: &OAuth) -> Result<(), Error> {
    let response = delete(
        &format!("{}/anime/{}/my_list_status", API_URL, anime_id),
        auth,
    )
    .await?;
    if response.status.is_success() {
        Ok(())
    } else {
        Err(Error::HttpError(response.status))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetUserAnimeListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortStyle>,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
}

pub async fn get_user_anime_list<U: ToString>(
    user: U,
    query: &GetUserAnimeListQuery,
    auth: &OAuth,
) -> Result<Page<Anime>, Error> {
    let response = get(
        &format!(
            "{}/users/{}/animelist?{}",
            API_URL,
            user.to_string(),
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::anime::tests::*;

    #[tokio::test]
    #[ignore]
    async fn test_delete_anime_from_list() {
        let auth = crate::auth::tests::get_auth();
        let anime = get_anime("God of High School", &auth).await.unwrap();
        delete_anime_from_list(anime.id, &auth).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_anime_list() {
        let auth = crate::auth::tests::get_auth();
        let query = UpdateUserAnimeListStatusQuery {
            status: Some(UserWatchStatus::Completed),
            is_rewatching: None,
            score: Some(8),
            num_watched_episodes: Some(13),
            priority: None,
            num_times_rewatched: None,
            rewatch_value: None,
            tags: None,
            comments: None,
        };

        let anime = get_anime(
            "Yahari Ore no Seishun Love Comedy wa Machigatteiru. Kan",
            &auth,
        )
        .await
        .unwrap();

        let result = update_anime_list_status(anime.id, &query, &auth)
            .await
            .unwrap();
        println!("{:#?}", result);
        assert_eq!(result.num_episodes_watched, 5);
    }

    #[tokio::test]
    async fn test_get_user_anime_list() {
        let auth = crate::auth::tests::get_auth();
        let query = GetUserAnimeListQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            status: None,
            sort: Some(SortStyle::ListScore),
            limit: 100,
            offset: 0,
            nsfw: true,
        };
        let result = get_user_anime_list("@me", &query, &auth).await.unwrap();

        print!("{:#?}", result);

        assert!(result.data.len() > 0);
    }
}
