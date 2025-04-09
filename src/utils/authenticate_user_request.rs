use crate::packages::errors::AuthenticateError;
use crate::packages::errors::Error;
use crate::packages::jwt_token;
use crate::packages::jwt_token::JwtUser;
use crate::packages::settings::SETTINGS;

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

impl<S> FromRequestParts<S> for JwtUser
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticateError::InvalidToken)?;

        let secret = &SETTINGS.read().secrets.signing_key;
        let token_data = jwt_token::decode(bearer.token(), secret).map_err(|_| AuthenticateError::InvalidToken)?;

        Ok(JwtUser::from_claim(token_data.claims))
    }
}
