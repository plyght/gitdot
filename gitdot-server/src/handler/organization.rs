mod add_member;
mod create_organization;
mod get_organization;
mod list_organization_members;
mod list_organization_repositories;
mod list_organizations;
mod update_member;
mod upload_organization_image;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, patch, post},
};

use crate::app::AppState;

use add_member::add_member;
use create_organization::create_organization;
use get_organization::get_organization;
use list_organization_members::list_organization_members;
use list_organization_repositories::list_organization_repositories;
use list_organizations::list_organizations;
use update_member::update_member;
use upload_organization_image::upload_organization_image;

pub fn create_organization_router() -> Router<AppState> {
    Router::new()
        .route("/organizations", get(list_organizations))
        .route(
            "/organization/{org_name}",
            get(get_organization).post(create_organization),
        )
        .route(
            "/organization/{org_name}/image",
            post(upload_organization_image).layer(DefaultBodyLimit::max(5 * 1024 * 1024)),
        )
        .route("/organization/{org_name}/member", post(add_member))
        .route(
            "/organization/{org_name}/member/{member_id}",
            patch(update_member),
        )
        .route(
            "/organization/{org_name}/members",
            get(list_organization_members),
        )
        .route(
            "/organization/{org_name}/repositories",
            get(list_organization_repositories),
        )
}
