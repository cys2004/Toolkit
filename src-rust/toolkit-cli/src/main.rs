use std::io::{stdin, stdout, Write};
use confique::Config;
use octocrab::Octocrab;

use config::AppConfig;
use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::models::GitHubRepo;

mod config;

/**
 * Set the contributor repository to the given repository.
 */
async fn set_contributor_repo(repo: GitHubRepo) {
    println!("Setting the contributor repository to: {}", repo.get_full_name());
    // TODO: Really set the contributor repository
}

/**
 * Automatically fork the upstream repository or use another forked repository.
 */
async fn create_fork(github: &GitHubApi, upstream_repo: GitHubRepo) {
    print!("Do you want to use another fork or create a new fork? (y/n) ");
    let mut user_input = String::new();
    let _= stdout().flush();
    stdin().read_line(&mut user_input).unwrap_or(0);
    match user_input.to_lowercase().trim() {
        "y" | "yes" => {
            let config = AppConfig::builder()
                // Used to get the GitHub username and token while developing and testing the CLI
                .file("config/cli.dev.toml")
                // The proper config when in production
                .file("config/config.toml")
                .load()
                .expect("Failed to load the configuration file.");
            let owner = config.github.username;
            println!("Please enter the name of the owner of the forked repository (default: {}):", owner);
            let mut fork_owner = String::new();
            stdin().read_line(&mut fork_owner).unwrap_or(0);
            fork_owner = fork_owner.trim().to_string();
            if fork_owner.is_empty() {
                fork_owner = owner;
            }
            let repo = upstream_repo.name;
            println!("Please enter the name of the forked repository (default: {}):", repo);
            let mut fork_repo = String::new();
            stdin().read_line(&mut fork_repo).unwrap_or(0);
            fork_repo = fork_repo.trim().to_string();
            if fork_repo.is_empty() {
                fork_repo = repo;
            }
            println!("Creating a fork with the owner: {} and the repository name: {}", fork_owner, fork_repo);
        }
        _ => {
            println!("You should use a fork of the upstream repository to contribute. Exiting...");
            std::process::exit(0);
        }
    }
}

/**
 * Check if the user has a forked repository of the upstream repository.
 */
async fn fork_check(github: &GitHubApi, upstream_repo: GitHubRepo) {
    let user_fork = github.get_user_fork(upstream_repo.clone()).await;
    match user_fork {
        Ok(fork) => {
            println!("You have forked the upstream repository: {}", upstream_repo.get_full_name());
            println!("Your fork repo have the name: {}", fork.get_full_name());
            print!("Do you want to use this forked repository as the contributor repository? (y/n) ");
            let mut user_input = String::new();
            let _= stdout().flush();
            stdin().read_line(&mut user_input).unwrap_or(0);
            match user_input.to_lowercase().trim() {
                "y" | "yes" => {
                    set_contributor_repo(fork).await;
                }
                _ => {
                    create_fork(github, upstream_repo).await;
                }
            }
        }
        Err(_) => {
            println!("You don't have a fork of the upstream repository: {}", upstream_repo.get_full_name());
            create_fork(github, upstream_repo).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::builder()
        // Used to get the GitHub username and token while developing and testing the CLI
        .file("config/cli.dev.toml")
        // The proper config when in production
        .file("config/config.toml")
        .load()
        .expect("Failed to load the configuration file.");

    let octocrab = Octocrab::builder().personal_token(
        config.github.token
    ).build().unwrap();

    let github = GitHubApi::new(
        config.github.username,
        octocrab
    );
    if !github.verify_user().await {
        eprintln!("Failed to verify the GitHub user.");
        std::process::exit(1);
    }

    println!("Please select the upstream repository you want to work with:");
    println!("1. FOSScope/Articles - 开源观察原创文章与中文转载文章源文件");
    println!("2. FOSScope/TranslateProject - 开源观察翻译项目");

    let mut user_input = String::new();
    let mut valid_input = false;
    let mut upstream_repo = GitHubRepo { owner: "".to_string(), name: "".to_string() };
    while !valid_input {
        stdin().read_line(&mut user_input).unwrap_or(0);
        upstream_repo = match user_input.trim() {
            "1" => {
                valid_input = true;
                GitHubRepo::new("FOSScope".to_string(), "Articles".to_string())
            }
            "2" => {
                valid_input = true;
                GitHubRepo::new("FOSScope".to_string(), "TranslateProject".to_string())
            }
            _ => {
                user_input.clear();
                eprintln!("Invalid input. Please select an option from the list.");
                continue;
            }
        }
    }
    user_input.clear();
    if upstream_repo.owner.is_empty() || upstream_repo.name.is_empty() {
        eprintln!("Failed to set the upstream repository.");
        std::process::exit(1);
    }

    fork_check(&github, upstream_repo).await;
}
