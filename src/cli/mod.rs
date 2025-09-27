//! CLI argument parsing and command handling

use crate::config::Config;
use crate::error::{Error, Result};
use clap::Parser;
// use std::env; // Removed - no longer needed

pub mod commands;

/// CLI output structure for JSON format
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CLIOutput {
    pub success: bool,
    pub data: Option<CLISuccessData>,
    pub error: Option<CLIErrorData>,
}

/// Success data structure for CLI JSON output
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CLISuccessData {
    pub extracted_text: String,
    pub file_name: String,
    pub file_size: i64,
    pub processing_time_ms: i64,
    pub confidence: Option<f64>,
}

/// Error data structure for CLI JSON output
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CLIErrorData {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    pub details: Option<String>,
}

impl CLIOutput {
    /// Validate CLI output structure
    pub fn validate(&self) -> Result<()> {
        // Ensure mutual exclusivity of data and error
        match (self.success, &self.data, &self.error) {
            (true, Some(_), None) => Ok(()),
            (false, None, Some(_)) => Ok(()),
            _ => Err(Error::Validation("CLI output must have either data (success=true) or error (success=false), not both".to_string())),
        }
    }
}

#[derive(Parser)]
#[command(
    name = "paperless-ngx-ocr2",
    author = "fzymgc-house",
    version,
    about = "OCR CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction",
    long_about = "A command-line tool for extracting text from PDF and image files using Mistral AI's OCR capabilities. Supports TOML configuration, 12-factor app principles, and provides both human-readable and JSON output formats."
)]
pub struct Cli {
    /// File to process for OCR
    #[arg(short, long, help = "Path to the PDF or image file to process", value_name = "FILE")]
    pub file: Option<String>,

    /// API key for Mistral AI
    #[arg(short, long, env = "PAPERLESS_OCR_API_KEY", help = "Mistral AI API key (can also be set via environment variable)", value_name = "KEY")]
    pub api_key: Option<String>,

    /// API base URL
    #[arg(long, env = "PAPERLESS_OCR_API_BASE_URL", help = "Mistral AI API base URL", value_name = "URL", default_value = "https://api.mistral.ai")]
    pub api_base_url: Option<String>,

    /// Output format as JSON
    #[arg(long, help = "Output result in JSON format instead of human-readable text")]
    pub json: bool,

    /// Verbose output
    #[arg(short, long, help = "Enable verbose logging output")]
    pub verbose: bool,

    /// Custom configuration file path
    #[arg(long, help = "Path to custom configuration file", value_name = "PATH")]
    pub config: Option<String>,

    /// Generate shell completion scripts
    #[arg(long, help = "Generate shell completion scripts for the specified shell", value_name = "SHELL")]
    pub completions: Option<String>,
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<()> {
        // Handle completion generation first
        if let Some(ref shell) = self.completions {
            return self.generate_completion_script(shell);
        }

        // Validate CLI arguments first
        self.validate()?;

        // Initialize logging
        crate::init_logging(self.verbose)?;

        tracing::debug!("CLI arguments parsed: file={:?}, json={}, verbose={}", self.file, self.json, self.verbose);

        // Load configuration - use custom path if provided, otherwise use default search
        let mut config = if let Some(ref config_path) = self.config { Config::load_from_path(config_path)? } else { Config::load_without_validation()? };

        // Override config with CLI arguments
        if let Some(ref api_key) = self.api_key {
            config.api_key = api_key.clone();
        }

        if let Some(ref api_base_url) = self.api_base_url {
            config.api_base_url = api_base_url.clone();
        }

        // Validate final configuration after all overrides
        config.validate()?;

        tracing::debug!("Configuration loaded and validated");

        // Check if file is provided
        let file = self.file.as_ref().ok_or_else(|| Error::Validation("File path is required for OCR processing".to_string()))?;

        // Process the file using commands module
        match commands::process_ocr_command(file, &config, self.json, self.verbose).await {
            Ok(output) => {
                // Output result to stdout (constitutional requirement)
                println!("{}", output);
                Ok(())
            }
            Err(e) => {
                // Handle error output format
                if self.json {
                    // JSON error output
                    let json_error = CLIOutput {
                        success: false,
                        data: None,
                        error: Some(CLIErrorData { error_type: e.error_type().to_string(), message: e.user_message(), details: Some(e.to_string()) }),
                    };
                    println!("{}", serde_json::to_string_pretty(&json_error).unwrap_or_else(|_| "{}".to_string()));
                } else {
                    // Human-readable error output goes to stderr
                    eprintln!("Error: {}", e.user_message());
                }

                // Return the original error
                Err(e)
            }
        }
    }

    /// Validate CLI arguments
    pub fn validate(&self) -> Result<()> {
        // If generating completions, file is not required
        if self.completions.is_some() {
            return Ok(());
        }

        // For OCR processing, file is required
        if self.file.is_none() {
            return Err(Error::Validation("File path is required for OCR processing".to_string()));
        }

        // Validate file argument if provided
        if let Some(ref file) = self.file {
            if file.is_empty() {
                return Err(Error::Validation("File path cannot be empty".to_string()));
            }
        }

        // Validate API key if provided
        if let Some(ref api_key) = self.api_key {
            if api_key.is_empty() {
                return Err(Error::Config("API key cannot be empty".to_string()));
            }
        }

        // Validate API base URL if provided
        if let Some(ref api_base_url) = self.api_base_url {
            if api_base_url.is_empty() {
                return Err(Error::Config("API base URL cannot be empty".to_string()));
            }
        }

        Ok(())
    }

    /// Generate shell completion script
    fn generate_completion_script(&self, shell: &str) -> Result<()> {
        let bin_name = "paperless-ngx-ocr2";

        match shell.to_lowercase().as_str() {
            "bash" => self.generate_bash_completion(bin_name),
            "zsh" => self.generate_zsh_completion(bin_name),
            "fish" => self.generate_fish_completion(bin_name),
            "powershell" | "ps1" => self.generate_powershell_completion(bin_name),
            _ => Err(Error::Config(format!("Unsupported shell: {}. Supported shells: bash, zsh, fish, powershell", shell))),
        }
    }

    /// Generate bash completion script
    fn generate_bash_completion(&self, bin_name: &str) -> Result<()> {
        println!(
            r#"# Bash completion for {}
complete -F _paperless_ngx_ocr2_completion {}

_paperless_ngx_ocr2_completion() {{
    local cur prev opts
    COMPREPLY=()
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"

    opts="-f --file -a --api-key --api-base-url --config --json -v --verbose -h --help -V --version --completions"

    case "${{prev}}" in
        -f|--file)
            COMPREPLY=( $(compgen -f -- "$cur") )
            return 0
            ;;
        -a|--api-key)
            return 0
            ;;
        --api-base-url)
            return 0
            ;;
        --config)
            COMPREPLY=( $(compgen -f -- "$cur") )
            return 0
            ;;
        --completions)
            COMPREPLY=( $(compgen -W "bash zsh fish powershell" -- "$cur") )
            return 0
            ;;
    esac

    if [[ $cur == -* ]]; then
        COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
        return 0
    fi
}}"#,
            bin_name, bin_name
        );
        Ok(())
    }

    /// Generate zsh completion script
    fn generate_zsh_completion(&self, bin_name: &str) -> Result<()> {
        println!(
            r#"#compdef {}
local -a opts
opts=(
    '(-f --file)'{{-f,--file}}'[Path to the PDF or image file to process]:file:_files'
    '(-a --api-key)'{{-a,--api-key}}'[Mistral AI API key]:key:'
    '--api-base-url[Mistral AI API base URL]:url:'
    '--config[Path to custom configuration file]:file:_files'
    '--json[Output result in JSON format instead of human-readable text]'
    '(-v --verbose)'{{-v,--verbose}}'[Enable verbose logging output]'
    '--completions[Generate shell completion scripts for the specified shell]:shell:(bash zsh fish powershell)'
    '(-h --help)'{{-h,--help}}'[Print help]'
    '(-V --version)'{{-V,--version}}'[Print version]'
)
_arguments $opts"#,
            bin_name
        );
        Ok(())
    }

    /// Generate fish completion script
    fn generate_fish_completion(&self, bin_name: &str) -> Result<()> {
        println!(
            r#"# Fish completion for {}
complete -c {} -s f -l file -d "Path to the PDF or image file to process" -r
complete -c {} -s a -l api-key -d "Mistral AI API key" -r
complete -c {} -l api-base-url -d "Mistral AI API base URL" -r
complete -c {} -l config -d "Path to custom configuration file" -r -F
complete -c {} -l json -d "Output result in JSON format instead of human-readable text"
complete -c {} -s v -l verbose -d "Enable verbose logging output"
complete -c {} -l completions -d "Generate shell completion scripts for the specified shell" -x -a "bash zsh fish powershell"
complete -c {} -s h -l help -d "Print help"
complete -c {} -s V -l version -d "Print version""#,
            bin_name, bin_name, bin_name, bin_name, bin_name, bin_name, bin_name, bin_name, bin_name, bin_name
        );
        Ok(())
    }

    /// Generate PowerShell completion script
    fn generate_powershell_completion(&self, bin_name: &str) -> Result<()> {
        println!(
            r#"# PowerShell completion for {}
Register-ArgumentCompleter -CommandName {} -ScriptBlock {{
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)

    $completions = @()

    switch ($parameterName) {{
        'File' {{
            $completions = Get-ChildItem -Path . -Name | Where-Object {{ $_ -like "$wordToComplete*" }}
        }}
        'Config' {{
            $completions = Get-ChildItem -Path . -Name -Include "*.toml" | Where-Object {{ $_ -like "$wordToComplete*" }}
        }}
        'Completions' {{
            $completions = @('bash', 'zsh', 'fish', 'powershell') | Where-Object {{ $_ -like "$wordToComplete*" }}
        }}
        default {{
            $completions = @('-f', '--file', '-a', '--api-key', '--api-base-url', '--config', '--json', '-v', '--verbose', '--completions', '-h', '--help', '-V', '--version') | Where-Object {{ $_ -like "$wordToComplete*" }}
        }}
    }}

    return $completions
}}"#,
            bin_name, bin_name
        );
        Ok(())
    }
}
