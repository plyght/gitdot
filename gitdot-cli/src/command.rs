mod login;
mod status;

use clap::Parser;

pub use login::LoginArgs;
pub use status::StatusArgs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Authenticate with gitdot OAuth and store credentials locally
    Login(LoginArgs),

    /// Display the current authentication state and logged-in user
    Status(StatusArgs),
    // TODO: re-enable as these features ship.
    // Save(SaveArgs),
    // Review(ReviewArgs),
    // Ci(CiArgs),
    // Runner(RunnerArgs),
}
