use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};

use serde::{Deserialize, Serialize};
use warp::{reject, Filter, Rejection};

#[derive(Debug)]
struct AuthRejection;

#[derive(Debug, Serialize, Deserialize)]
struct ClaimPayload {
  sub: String,
  user_id: String,
  exp: usize,
}

// TODO: create useful rejection for this
impl reject::Reject for AuthRejection {}

// Middleware for validating JWT
pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
  warp::header::<String>("Authorization").and_then(|token: String| {
    async move {
      let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET should be provided");
      let secret = secret.as_bytes();

      if token.len() < 7 {
        return Err(reject::custom(AuthRejection));
      };

      // ignore the Bearer part
      let token = &token[7..];

      let payload = match decode::<ClaimPayload>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
      ) {
        Ok(p) => Ok(p.claims.user_id),
        Err(err) => match *err.kind() {
          ErrorKind::InvalidToken => Err(reject::custom(AuthRejection)),
          ErrorKind::InvalidIssuer => Err(reject::custom(AuthRejection)),
          _ => Err(reject::custom(AuthRejection)), // idk
        },
      };

      payload
    }
  })
}
