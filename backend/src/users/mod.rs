use axum::extract::FromRequestParts;
use josekit::{
    jwk::JwkSet,
    jws::alg::eddsa::EddsaJwsAlgorithm::Eddsa,
    jwt::{self, JwtContext},
};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::users::errors::AuthError;

mod errors;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub iat: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    // pub image: null,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub id: String,
    pub sub: String,
    pub exp: i32,
    pub iss: String,
    pub aud: String,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Claims> for User {
    fn from(value: Claims) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            email_verified: value.email_verified,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    // TODO: Cleanup
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::Unauthorized)?;

        let token = bearer
            .strip_prefix("Bearer ")
            .ok_or(AuthError::Unauthorized)?;

        let header = jwt::decode_header(token)?;

        // TODO: Remove hardcoded url
        let jwk_body = reqwest::get("http://localhost:5173/api/auth/jwks")
            .await?
            .text()
            .await?;

        let key_set = JwkSet::from_bytes(jwk_body.as_bytes())?;

        let verifier = key_set
            .keys()
            .iter()
            .find_map(|k| {
                let jwt_kid = header.claim("kid").and_then(|v| v.as_str());

                if k.key_id()? == jwt_kid? {
                    Some(Eddsa.verifier_from_jwk(k))
                } else {
                    None
                }
            })
            .transpose()?
            .ok_or(AuthError::Unauthorized)?;

        let (payload, _) = JwtContext::new().decode_with_verifier(token, &verifier)?;

        let claims: Claims = serde_json::from_str(&payload.to_string())?;

        Ok(User::from(claims))
    }
}
