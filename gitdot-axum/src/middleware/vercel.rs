use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use gitdot_core::dto::JwtClaims;

use crate::config::VercelOidcConfig;

pub async fn verify_vercel_oidc(
    State(config): State<VercelOidcConfig>,
    mut request: Request,
    next: Next,
) -> Response {
    if verify(&request, &config).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // TODO: this is purely for IP based rate limiting as we use
    // SSR in the frontend and all incoming IPs are from the same egress IP.
    // For a proper fix, implement account based rate limiting.
    promote_client_ip(&mut request);

    next.run(request).await
}

fn promote_client_ip(request: &mut Request) {
    let Some(ip) = request.headers().get("x-gitdot-client-ip").cloned() else {
        return;
    };
    let headers = request.headers_mut();
    headers.insert("x-forwarded-for", ip.clone());
    headers.insert("x-real-ip", ip);
}

fn verify(request: &Request, config: &VercelOidcConfig) -> Result<(), ()> {
    let token = request
        .headers()
        .get("x-vercel-oidc-token")
        .and_then(|v| v.to_str().ok())
        .ok_or(())?;

    let header = decode_header(token).map_err(|_| ())?;
    let kid = header.kid.ok_or(())?;
    let jwk = config.jwks.find(&kid).ok_or(())?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| ())?;

    let audience = config.issuer.replace("oidc.vercel.com", "vercel.com");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&audience]);
    validation.set_issuer(&[&config.issuer]);

    decode::<JwtClaims>(token, &key, &validation).map_err(|_| ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::body::Body;

    use super::*;

    fn request_with(headers: &[(&str, &str)]) -> Request {
        let mut builder = Request::builder();
        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }
        builder.body(Body::empty()).unwrap()
    }

    fn header<'a>(request: &'a Request, name: &str) -> Option<&'a str> {
        request.headers().get(name).map(|v| v.to_str().unwrap())
    }

    #[test]
    fn overwrites_forwarded_headers_with_client_ip() {
        // Stale x-forwarded-for (the Vercel/GCP egress value) must be replaced.
        let mut request = request_with(&[
            ("x-gitdot-client-ip", "203.0.113.7"),
            ("x-forwarded-for", "10.0.0.1"),
        ]);

        promote_client_ip(&mut request);

        assert_eq!(header(&request, "x-forwarded-for"), Some("203.0.113.7"));
        assert_eq!(header(&request, "x-real-ip"), Some("203.0.113.7"));
    }

    #[test]
    fn sets_both_headers_when_only_client_ip_present() {
        let mut request = request_with(&[("x-gitdot-client-ip", "198.51.100.9")]);

        promote_client_ip(&mut request);

        assert_eq!(header(&request, "x-forwarded-for"), Some("198.51.100.9"));
        assert_eq!(header(&request, "x-real-ip"), Some("198.51.100.9"));
    }

    #[test]
    fn leaves_headers_untouched_when_client_ip_absent() {
        let mut request = request_with(&[("x-forwarded-for", "10.0.0.1")]);

        promote_client_ip(&mut request);

        // No spoofable promotion happened: existing value kept, x-real-ip unset.
        assert_eq!(header(&request, "x-forwarded-for"), Some("10.0.0.1"));
        assert_eq!(header(&request, "x-real-ip"), None);
    }
}
