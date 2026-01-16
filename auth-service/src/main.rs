use authservice::Application;

#[tokio::main]
async fn main() {
    let application = Application::build("0.0.0.0:3000")
        .await
        .expect("failed to build app");

    application.run().await.expect("failed to run app")
}
