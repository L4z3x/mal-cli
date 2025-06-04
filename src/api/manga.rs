use super::model::*;
use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::OAuth;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaListQuery {
    pub q: String,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_manga_list(query: &GetMangaListQuery, auth: &OAuth) -> Result<Page<Manga>, Error> {
    let response = get(
        &format! {"{}/manga?{}", API_URL, serde_urlencoded::to_string(query)?},
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaDetailQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    pub nsfw: bool,
}

pub async fn get_manga_details(
    manga_id: u64,
    query: &GetMangaDetailQuery,
    auth: &OAuth,
) -> Result<Manga, Error> {
    let response = get(
        &format!(
            "{}/manga/{}?{}",
            API_URL,
            manga_id,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetMangaRankingQuery {
    pub ranking_type: MangaRankingType,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_manga_ranking(
    query: &GetMangaRankingQuery,
    auth: &OAuth,
) -> Result<Ranking<RankingMangaPair>, Error> {
    let response = get(
        &format!(
            "{}/manga/ranking?{}",
            API_URL,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub async fn get_manga<T: ToString>(q: T, auth: &OAuth) -> Result<Manga, Error> {
        let manga_query = GetMangaListQuery {
            q: q.to_string(),
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let manga_list = get_manga_list(&manga_query, &auth).await.unwrap();
        let manga = manga_list.data.first().unwrap().node.clone();
        Ok(manga)
    }

    #[tokio::test]
    async fn test_get_manga_list() {
        let auth = crate::auth::tests::get_auth();
        let query = GetMangaListQuery {
            q: "Kaguya-Sama Wa Kokurasetai".to_string(),
            limit: 2,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let result = get_manga_list(&query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }

    #[tokio::test]
    async fn test_get_manga_details() {
        let auth = crate::auth::tests::get_auth();
        let query = GetMangaDetailQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            nsfw: false,
        };

        let manga = get_manga("Kaguya-Sama Wa Kokurasetai", &auth)
            .await
            .unwrap();
        let result = get_manga_details(manga.id, &query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert_eq!(result.title, manga.title);
    }

    #[tokio::test]
    async fn test_get_manga_ranking() {
        let auth = crate::auth::tests::get_auth();
        let query = GetMangaRankingQuery {
            ranking_type: MangaRankingType::All,
            limit: 100,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let result = get_manga_ranking(&query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }
}
