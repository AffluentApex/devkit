mod check;
mod complexity;
mod dependencies;
mod git;

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "devkit",
    about = "A comprehensive developer toolkit",
    version,
    author,
    after_help = "Example commands:\n  devkit check              # Check development environment\n  devkit git .              # Analyze current git repository\n  devkit complexity src     # Analyze code complexity\n",
    before_help = "🛠️  Welcome to DevKit! 🚀"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Git repository statistics
    Git {
        /// Path to the git repository
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Calculate code complexity metrics
    Complexity {
        /// Path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Analyze project dependencies
    Dependencies {
        /// Path to project
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Create a new project from template
    New {
        /// Project name
        name: String,
        /// Template to use (rust, node, python)
        #[arg(default_value = "rust")]
        template: String,
    },
    /// Check development environment
    Check {
        /// Tools to check (comma-separated)
        #[arg(default_value = "git,rust,node,python")]
        tools: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("\n{}", "🛠️  Welcome to DevKit! 🚀".green().bold());
            println!("\n{}", "Quick Start Guide:".yellow().bold());
            println!("  1. {} Check your development environment:", "►".cyan());
            println!("     {}", "devkit check".bright_black());
            println!("  2. {} Analyze your git repository:", "►".cyan());
            println!("     {}", "devkit git .".bright_black());
            println!("  3. {} Calculate code complexity:", "►".cyan());
            println!("     {}", "devkit complexity src".bright_black());
            println!("  4. {} Analyze project dependencies:", "►".cyan());
            println!("     {}", "devkit dependencies .".bright_black());
            println!("\n{}", "For more information, run:".yellow());
            println!("  {}", "devkit --help".bright_black());
            println!("  {}", "devkit <command> --help".bright_black());
        }
        Some(Commands::Git { path }) => {
            println!("{}", "\n🔍 Analyzing Git repository...".green().bold());
            match git::GitAnalyzer::new(&path) {
                Ok(analyzer) => {
                    match analyzer.analyze() {
                        Ok(stats) => {
                            println!("\n📊 Repository Statistics:");
                            println!("├─ Total commits: {}", stats.total_commits.to_string().cyan());
                            println!("└─ Branch count: {}", stats.branch_count.to_string().cyan());
                            
                            if let Some(latest) = stats.latest_commit {
                                println!("\n🔥 Latest Commit:");
                                println!("├─ Author: {}", latest.author.cyan());
                                println!("├─ Time: {}", format_time(latest.time).cyan());
                                println!("└─ Message: {}", latest.message.trim().cyan());
                            }
                        }
                        Err(e) => eprintln!("{}", format!("❌ Error analyzing repository: {}", e).red()),
                    }
                }
                Err(e) => eprintln!("{}", format!("❌ Error opening repository: {}", e).red()),
            }
        }
        Some(Commands::Complexity { path }) => {
            println!("{}", "\n📊 Analyzing code complexity...".green().bold());
            let analyzer = complexity::ComplexityAnalyzer::new();
            let stats = analyzer.analyze(&path);
            
            println!("\n📈 Complexity Analysis:");
            println!("├─ Files analyzed: {}", stats.file_count.to_string().cyan());
            println!("├─ Total lines: {}", stats.total_lines.to_string().cyan());
            println!("├─ Code lines: {}", stats.code_lines.to_string().cyan());
            println!("├─ Comment lines: {}", stats.comment_lines.to_string().cyan());
            println!("└─ Blank lines: {}", stats.blank_lines.to_string().cyan());
            
            if stats.total_lines > 0 {
                let code_ratio = (stats.code_lines as f64 / stats.total_lines as f64 * 100.0) as u32;
                let comment_ratio = (stats.comment_lines as f64 / stats.total_lines as f64 * 100.0) as u32;
                
                println!("\n📊 Code Distribution:");
                println!("├─ Code: {}%", format_percentage_bar(code_ratio, 20));
                println!("└─ Comments: {}%", format_percentage_bar(comment_ratio, 20));
            }
        }
        Some(Commands::Dependencies { path }) => {
            println!("{}", "\n📦 Analyzing dependencies...".green().bold());
            let analyzer = dependencies::DependencyAnalyzer::new();
            let stats = analyzer.analyze(&path);
            println!("\n📋 Dependency Analysis:");
            println!("└─ Total dependencies: {}", stats.total_deps.to_string().cyan());
        }
        Some(Commands::New { name, template }) => {
            println!("{}", format!("\n🚀 Creating new {} project: {}", template, name).green().bold());
            // TODO: Implement project scaffolding
            println!("Coming soon! 🎉");
        }
        Some(Commands::Check { tools }) => {
            println!("{}", "\n🔍 Checking development environment...".green().bold());
            let checker = check::EnvironmentChecker::new();
            let results = checker.check_tools(&tools);
            
            println!("\n🛠️  Environment Check Results:");
            for tool in results {
                let status = if tool.installed {
                    "✓".green()
                } else {
                    "✗".red()
                };
                
                print!("{} {} ", status, tool.name.bold());
                if let Some(version) = tool.version {
                    println!("({})", version.trim().cyan());
                } else {
                    println!("{}", "not found".red());
                }
            }
        }
    }
}

fn format_time(time: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let diff = now - time;
    if diff < 60 {
        format!("just now")
    } else if diff < 3600 {
        format!("{} minutes ago", diff / 60)
    } else if diff < 86400 {
        format!("{} hours ago", diff / 3600)
    } else {
        format!("{} days ago", diff / 86400)
    }
}

fn format_percentage_bar(percentage: u32, width: usize) -> String {
    let filled_width = (percentage as f64 / 100.0 * width as f64) as usize;
    let empty_width = width - filled_width;
    
    format!("{}{} {}%",
        "█".repeat(filled_width).cyan(),
        "░".repeat(empty_width),
        percentage
    )
}
