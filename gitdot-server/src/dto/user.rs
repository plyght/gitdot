use gitdot_api::resource::user as api;
use gitdot_core::dto::{GetCurrentUserResponse, UserResponse};

use super::IntoApi;

impl IntoApi for UserResponse {
    type ApiType = api::UserResource;
    fn into_api(self) -> Self::ApiType {
        api::UserResource {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
            location: self.location,
            readme: self.readme,
            links: self.links,
            display_name: self.display_name,
        }
    }
}

impl IntoApi for GetCurrentUserResponse {
    type ApiType = api::CurrentUserResource;
    fn into_api(self) -> Self::ApiType {
        api::CurrentUserResource {
            user: self.user.into_api(),
            memberships: self.memberships.into_api(),
        }
    }
}
