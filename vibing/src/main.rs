use vibing::{cli::matcher, env, error::handle};

#[tokio::main]
async fn main() {
    println!("Vibing ~");
    env::load();
    // FIXME: Still panicks ig user cancels the credentials read operation
    let result = matcher::handle_cli_args().await;
    if let Err(result_err) = &result {
        println!("Error thrown");
        println!("{result_err:#?}");
    }
    if let Err(err) = result {
        handle(err);
    }
}
