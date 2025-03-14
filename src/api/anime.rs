use super::model::*;
use super::Error;
use super::{get, handle_response, API_URL};
use crate::auth::OAuth;
use serde::Serialize;

/// Get Anime List Request
#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeListQuery {
    pub q: String,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_anime_list(query: &GetAnimeListQuery, auth: &OAuth) -> Result<Page<Anime>, Error> {
    let response = get(
        &format!("{}/anime?{}", API_URL, serde_urlencoded::to_string(query)?),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeDetailQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    pub nsfw: bool,
}

pub async fn get_anime_details(
    anime_id: u64,
    query: &GetAnimeDetailQuery,
    auth: &OAuth,
) -> Result<Anime, Error> {
    let response = get(
        &format!(
            "{}/anime/{}?{}",
            API_URL,
            anime_id,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetAnimeRankingQuery {
    pub ranking_type: AnimeRankingType,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_anime_ranking(
    query: &GetAnimeRankingQuery,
    auth: &OAuth,
) -> Result<Ranking<RankingAnimePair>, Error> {
    let response = get(
        &format!(
            "{}/anime/ranking?{}",
            API_URL,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetSeasonalAnimeQuery {
    pub sort: Option<SortStyle>,
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_seasonal_anime(
    season: &AnimeSeason,
    query: &GetSeasonalAnimeQuery,
    auth: &OAuth,
) -> Result<Page<Anime>, Error> {
    let season_name: &'static str = season.season.clone().into();
    let response = get(
        &format!(
            "{}/anime/season/{}/{}?{}",
            API_URL,
            season.year,
            season_name,
            serde_urlencoded::to_string(query)?
        ),
        auth,
    )
    .await?;
    handle_response(&response)
}

#[derive(Clone, Debug, Serialize)]
pub struct GetSuggestedAnimeQuery {
    pub limit: u64,
    pub offset: u64,
    pub nsfw: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

pub async fn get_suggested_anime(
    query: &GetSuggestedAnimeQuery,
    auth: &OAuth,
) -> Result<Page<Anime>, Error> {
    let response = get(
        &format!(
            "{}/anime/suggestions?{}",
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

    pub async fn get_anime<T: ToString>(q: T, auth: &OAuth) -> Result<Anime, Error> {
        let anime_query = GetAnimeListQuery {
            q: q.to_string(),
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let anime_list = get_anime_list(&anime_query, &auth).await.unwrap();
        let anime = anime_list.data.get(0).unwrap().node.clone();
        Ok(anime)
    }

    #[tokio::test]
    async fn test_get_anime_list() {
        let mut auth = crate::auth::tests::get_auth();
        auth.refresh().unwrap();
        // let oauth = crate::auth::OAuth::get_auth(app_config::AppConfig::Load())
        let query = GetAnimeListQuery {
            q: "Code Geass".to_string(),
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let result = get_anime_list(&query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }

    #[tokio::test]
    async fn test_get_anime_details() {
        let auth = crate::auth::tests::get_auth();
        let query = GetAnimeDetailQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            nsfw: false,
        };

        let anime = get_anime("Cowboy Bebop", &auth).await.unwrap();
        let result = get_anime_details(anime.id, &query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert_eq!(result.title, anime.title);
    }

    #[tokio::test]
    async fn test_get_anime_ranking() {
        let auth = crate::auth::tests::get_auth();
        let query = GetAnimeRankingQuery {
            ranking_type: AnimeRankingType::All,
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let result = get_anime_ranking(&query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }
    #[tokio::test]
    async fn test_get_seasonal_anime() {
        let auth = crate::auth::tests::get_auth();
        let query = GetSeasonalAnimeQuery {
            sort: None,
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let season = AnimeSeason {
            year: 2020,
            season: Season::Summer,
        };
        let result = get_seasonal_anime(&season, &query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }
    #[tokio::test]
    async fn test_get_suggested_anime() {
        let auth = crate::auth::tests::get_auth();
        let query = GetSuggestedAnimeQuery {
            limit: 4,
            offset: 0,
            nsfw: false,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };
        let result = get_suggested_anime(&query, &auth).await.unwrap();
        println!("{:#?}", result);
        assert!(result.data.len() > 0);
    }
}
