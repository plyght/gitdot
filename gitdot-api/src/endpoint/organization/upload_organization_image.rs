use crate::endpoint::Endpoint;

pub struct UploadOrganizationImage;

impl Endpoint for UploadOrganizationImage {
    const PATH: &'static str = "/organization/{org_name}/image";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = ();
}
