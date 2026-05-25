use crate::dto::{GitignoreTemplate, LicenseTemplate};

pub fn gitignore_for(template: GitignoreTemplate) -> &'static str {
    match template {
        GitignoreTemplate::Rust => include_str!("../../templates/gitignore/rust.gitignore"),
        GitignoreTemplate::Node => include_str!("../../templates/gitignore/node.gitignore"),
        GitignoreTemplate::Python => include_str!("../../templates/gitignore/python.gitignore"),
        GitignoreTemplate::Go => include_str!("../../templates/gitignore/go.gitignore"),
    }
}

pub fn license_for(template: LicenseTemplate) -> &'static str {
    match template {
        LicenseTemplate::Mit => include_str!("../../templates/license/mit.txt"),
        LicenseTemplate::Apache2 => include_str!("../../templates/license/apache-2.0.txt"),
    }
}
