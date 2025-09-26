use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Only generate man page in release builds or when explicitly requested
    if env::var("PROFILE").unwrap() == "release" || env::var("GENERATE_MAN").is_ok() {
        generate_man_page()?;
    }
    
    
    Ok(())
}

fn generate_man_page() -> Result<()> {
    // Define the command using clap
    let cmd = clap::Command::new("paperless-ngx-ocr2")
        .version(env!("CARGO_PKG_VERSION"))
        .author("fzymgc-house")
        .about("OCR CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction")
        .long_about("A command-line tool for extracting text from PDF and image files using Mistral AI's OCR capabilities. Supports TOML configuration, 12-factor app principles, and provides both human-readable and JSON output formats.")
        .arg(
            clap::Arg::new("file")
                .short('f')
                .long("file")
                .help("Path to the PDF or image file to process")
                .value_name("FILE")
                .required(false)
        )
        .arg(
            clap::Arg::new("api-key")
                .short('a')
                .long("api-key")
                .help("Mistral AI API key (can also be set via environment variable)")
                .value_name("KEY")
                .env("PAPERLESS_OCR_API_KEY")
        )
        .arg(
            clap::Arg::new("api-base-url")
                .long("api-base-url")
                .help("Mistral AI API base URL")
                .value_name("URL")
                .env("PAPERLESS_OCR_API_BASE_URL")
                .default_value("https://api.mistral.ai")
        )
        .arg(
            clap::Arg::new("config")
                .long("config")
                .help("Path to custom configuration file")
                .value_name("PATH")
        )
        .arg(
            clap::Arg::new("json")
                .long("json")
                .help("Output result in JSON format instead of human-readable text")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("completions")
                .long("completions")
                .help("Generate shell completion scripts for the specified shell")
                .value_name("SHELL")
                .value_parser(["bash", "zsh", "fish", "powershell"])
        );

    // Generate the man page
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    // Write to output directory
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("paperless-ngx-ocr2.1"), &buffer)?;

    // Also write to project root for easy access
    fs::write("paperless-ngx-ocr2.1", &buffer)?;

    println!("cargo:warning=Man page generated: paperless-ngx-ocr2.1");
    Ok(())
}
