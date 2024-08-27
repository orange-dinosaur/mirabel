use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn set_cors() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            header::ACCESS_CONTROL_ALLOW_METHODS,
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::REFERER,
            header::ACCEPT_LANGUAGE,
            header::ACCESS_CONTROL_REQUEST_HEADERS,
            header::ACCESS_CONTROL_REQUEST_METHOD,
            header::CACHE_CONTROL,
        ])
        .allow_credentials(true);

    // get the allowed origins from the environment
    let allowed_origins = get_allowed_origins();

    if allowed_origins.is_empty() {
        cors.allow_origin(Any).allow_credentials(false)
    } else {
        cors.allow_origin(allowed_origins)
    }
}

fn get_allowed_origins() -> Vec<HeaderValue> {
    // get the allowed origins from the environment
    let allowed_origins_env = match std::env::var("ALLOWED_ORIGINS") {
        Ok(allowed_origins_env) => allowed_origins_env,
        Err(_) => {
            return Vec::<HeaderValue>::new();
        }
    };

    // split the string into a vector of String
    let mut allowed_origins: Vec<String> =
        allowed_origins_env.split("::").map(String::from).collect();

    // remove any empty strings
    allowed_origins.retain(|s| !s.is_empty());

    if allowed_origins.is_empty() {
        return Vec::<HeaderValue>::new();
    }

    // convert the vector of String to a vector of HeaderValue
    allowed_origins.iter().map(|s| s.parse().unwrap()).collect()
}
