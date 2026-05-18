mod build;
mod git_http;
mod internal;
mod migration;
mod organization;
mod question;
mod repository;
mod review;
mod runner;
mod task;
mod user;
mod webhook;

pub use git_http::*;
pub use internal::*;
pub use webhook::*;

pub trait IntoApi {
    type ApiType;
    fn into_api(self) -> Self::ApiType;
}

impl<T> IntoApi for Vec<T>
where
    T: IntoApi,
{
    type ApiType = Vec<T::ApiType>;
    fn into_api(self) -> Self::ApiType {
        self.into_iter().map(|item| item.into_api()).collect()
    }
}

impl<T> IntoApi for Option<T>
where
    T: IntoApi,
{
    type ApiType = Option<T::ApiType>;
    fn into_api(self) -> Self::ApiType {
        self.map(|item| item.into_api())
    }
}

impl<T> IntoApi for gitdot_core::dto::Page<T>
where
    T: IntoApi,
{
    type ApiType = gitdot_api::resource::Page<T::ApiType>;
    fn into_api(self) -> Self::ApiType {
        gitdot_api::resource::Page {
            data: self.data.into_iter().map(IntoApi::into_api).collect(),
            next_cursor: self.next_cursor,
        }
    }
}
