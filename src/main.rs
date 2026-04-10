use clap::Parser;
use vke_manager::cli::Cli;

#[tokio::main]
async fn main() {
    if let Err(error) = vke_manager::run(Cli::parse()).await {
        eprintln!("执行失败: {error:#}");
        std::process::exit(1);
    }
}
