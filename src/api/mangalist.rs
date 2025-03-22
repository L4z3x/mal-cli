use super::model::*;
use super::Error;
use super::{delete, get, handle_response, patch, API_URL};
use crate::auth::OAuth;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UpdateUserMangaStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserReadStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rereading: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_volumes_read: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_chapters_read: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_times_reread: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reread_value: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

pub async fn update_manga_list_status(
    manga_id: u64,
    update: &UpdateUserMangaStatus,
    auth: &OAuth,
) -> Result<UserMangaListStatus, Error> {
    let response = patch(
        &format!("{}/manga/{}/my_list_status", API_URL, manga_id),
        auth,
        update,
    )
    .await?;
    handle_response(&response)
}

pub async fn delete_manga_from_list(manga_id: u64, auth: &OAuth) -> Result<(), Error> {
    let response = delete(
        &format!("{}/manga/{}/my_list_status", API_URL, manga_id),
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
pub struct GetUserMangaListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserReadStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortStyle>,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
}

pub async fn get_user_manga_list<U: ToString>(
    user: U,
    query: &GetUserMangaListQuery,
    auth: &OAuth,
) -> Result<Page<Manga>, Error> {
    let response = get(
        &format!(
            "{}/users/{}/mangalist?{}",
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
mod test {
    use super::*;
    use crate::api::manga::tests::*;

    #[tokio::test]
    async fn test_delete_manga_from_list() {
        let auth = crate::auth::tests::get_auth();
        let manga = get_manga("Grand Blue", &auth).await.unwrap();
        delete_manga_from_list(manga.id, &auth).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_manga_list() {
        let auth = crate::auth::tests::get_auth();
        let query = UpdateUserMangaStatus {
            status: Some(UserReadStatus::Reading),
            is_rereading: None,
            score: Some(9),
            num_volumes_read: None,
            num_chapters_read: Some(62),
            priority: None,
            num_times_reread: None,
            reread_value: None,
            tags: None,
            comments: None,
        };
        let manga = get_manga("Grand Blue", &auth).await.unwrap();
        let result = update_manga_list_status(manga.id, &query, &auth)
            .await
            .unwrap();
        println!("{:#?}", result);
        assert_eq!(result.num_chapters_read, 62);
    }

    #[tokio::test]
    async fn test_get_user_manga_list() {
        let auth = crate::auth::tests::get_auth();
        let query = GetUserMangaListQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            status: None,
            sort: None,
            limit: 100,
            offset: 0,
            nsfw: true,
        };
        let result = get_user_manga_list("@me", &query, &auth).await.unwrap();

        print!("{:#?}", result);

        assert!(result.data.len() > 0);
    }
}
