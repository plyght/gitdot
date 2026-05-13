mod runner;
mod user;

pub use runner::RunnerConfig;
pub use user::UserConfig;

fn default_gitdot_server_url() -> String {
    "https://api.gitdot.io".to_string()
}

fn default_gitdot_web_url() -> String {
    "https://www.gitdot.io".to_string()
}

fn default_gitdot_auth_server_url() -> String {
    "https://auth.gitdot.io".to_string()
}

fn default_s2_server_url() -> String {
    "https://s2.gitdot.io".to_string()
}

fn default_num_executors() -> i8 {
    4
}
