use clap::{CommandFactory, Parser};
use paperless_ngx_ocr2::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments first
    let cli = Cli::parse();

    // Check if no arguments provided (except for help/version)
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // No arguments provided, show help
        let mut cmd = Cli::command();
        cmd.print_help()?;
        std::process::exit(0);
    }

    // Execute the CLI command with proper error handling
    match cli.execute().await {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // The CLI execute method will handle proper exit codes internally
            std::process::exit(e.exit_code());
        }
    }
}
