use message_actix::MessageApp;

fn main() -> Reslut<()> {
    std::env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();
    let app = MessageApp::new(8080);
    app.run();
}
