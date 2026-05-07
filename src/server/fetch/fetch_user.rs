use reqwest::{header::COOKIE};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub bio: String,
    pub username: String,
    pub online_status: String,
    pub friend_count: u8,
    pub follower_count: u8,
    pub following_count: u8,
    pub visits: u8,
    pub created_at: String,
}

static SESSION: &str = "session_token=0b6aa443c12d27a4867961a1c7c737fc5a2c65214c9decf207fccb9df47808dc";

pub async fn fetch_newest_user(client: &reqwest::Client) -> Option<String> {
    let mut low = 1u64;
    let mut high = 1u64;
    loop {
        let ok = client
            .get(format!("https://vortex.towerstats.com/api/users/{high}"))
            .header(COOKIE, SESSION)
            .send()
            .await
            .ok()?
            .status()
            .is_success();
        if !ok {break;}
        low = high;
        high *= 2;
    }
    while low + 1 < high {
        let mid = (low + high) / 2;

        let ok = client
            .get(format!("https://vortex.towerstats.com/api/users/{mid}"))
            .header(COOKIE, SESSION)
            .send()
            .await
            .ok()?
            .status()
            .is_success();

        if ok {low = mid;} else {high = mid;}
    } Some(low.to_string())
}

pub async fn fetch_user_name(client: &reqwest::Client, id: &str) -> Option<String> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.username)
}

pub async fn fetch_user_bio(client: &reqwest::Client, id: &str) -> Option<String> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.bio)
}

pub async fn fetch_user_status(client: &reqwest::Client, id: &str) -> Option<String> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.online_status)
}

pub async fn fetch_user_friends(client: &reqwest::Client, id: &str) -> Option<u8> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.friend_count)
}

pub async fn fetch_user_followers(client: &reqwest::Client, id: &str) -> Option<u8> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.follower_count)
}

pub async fn fetch_user_following(client: &reqwest::Client, id: &str) -> Option<u8> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.following_count)
}

pub async fn fetch_user_visits(client: &reqwest::Client, id: &str) -> Option<u8> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.visits)
}

pub async fn fetch_user_creation(client: &reqwest::Client, id: &str) -> Option<String> {
    client
        .get(format!("https://vortex.towerstats.com/api/users/{id}"))
        .header(
            COOKIE,
            SESSION,
        )
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json::<User>()
        .await
        .ok()
        .map(|u| u.created_at)
}
