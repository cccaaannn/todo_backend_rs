use crate::libs::jwt::{JWTTrait, JWT};
use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn api_key_authorize_middleware<B>(
    State(jwt): State<JWT>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let auth_header_split = auth_header.split(" ").collect::<Vec<&str>>();
    if auth_header_split.len() != 2 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = auth_header_split.get(1).unwrap();

    let verification_result = jwt.verify(token);
    if verification_result.is_err() {
        tracing::warn!("{}", verification_result.unwrap_err());
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
