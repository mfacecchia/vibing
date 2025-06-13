use vibing::{cli::{matcher}, env};

#[tokio::main]
async fn main() {
    println!("Vibing ~");
    env::load();
    matcher::handle_cli_args().await;
}
