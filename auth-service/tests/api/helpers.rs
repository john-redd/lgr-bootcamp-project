#![allow(clippy::let_underscore_future)]

use authservice::Application;
use std::error::Error;

pub async fn run_app() -> Result<String, Box<dyn Error>> {
    let application = Application::build("127.0.0.1:0")
        .await
        .expect("failed to build test app");

    let address = application.address.clone();

    let _ = tokio::spawn(application.run());

    Ok(address)
}
