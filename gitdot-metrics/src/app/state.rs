use std::sync::Arc;

use axum::extract::FromRef;
use jsonwebtoken::jwk::JwkSet;

use gitdot_axum::config::{AuthConfig, VercelOidcConfig};

use super::Settings;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,

    pub auth_config: AuthConfig,
    pub vercel_oidc_config: VercelOidcConfig,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> anyhow::Result<Self> {
        let vercel_jwks = {
            let jwks_url = format!("{}/.well-known/jwks", settings.vercel_oidc_url);
            reqwest::get(&jwks_url).await?.json::<JwkSet>().await?
        };
        let vercel_oidc_config = VercelOidcConfig {
            jwks: Arc::new(vercel_jwks),
            issuer: settings.vercel_oidc_url.clone(),
        };
        let auth_config = AuthConfig {
            public_key: settings.gitdot_public_key.clone(),
        };

        Ok(Self {
            settings,
            auth_config,
            vercel_oidc_config,
        })
    }
}
