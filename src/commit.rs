use inquire::{Confirm, MultiSelect, Select, Text};
use regex::Regex;
use crate::git_operations;

fn get_type() -> Result<String, String> {
    let options = vec![
        "fix",
        "feat",
        "chore",
        "docs",
        "style",
        "refactor",
        "perf",
        "test",
        "improvement"
    ];

    let selected_option = Select::new("Select commit type", options).prompt();

    match selected_option {
        Ok(choice) => Ok(format!("{}", choice)),
        Err(err) => Err(format!("An error occurred: {}", err))
    }
}

pub fn run_commit(conventional_commit: bool, ticket_prefix: bool) -> Result<(), String> {
    let repo = git_operations::get_repository()
        .ok_or("Failed to open repository")?;

    let changes = git_operations::get_untracked(&repo);
        
    if changes.is_empty() {
        println!("No untracked or modified files found.");
        return Ok(());
    }
    let selected_files = MultiSelect::new("Select changes to commit:", changes)
        .prompt()
        .map_err(|e| format!("An error occurred during selection: {}", e))?;

    if selected_files.is_empty() {
        println!("No files selected.");
        return Ok(());
    }

    let mut index = repo.index().map_err(|e| format!("Error accessing index: {}", e))?;
        
    let commit_type = if conventional_commit {
        get_type().map_err(|e| format!("An error occurred: {}", e))?
    } else {
        String::new()
    };

    let ticket = if ticket_prefix {
        let re = Regex::new(r"[A-Z]+-[0-9]+").unwrap();
        let branch = git_operations::get_current_branch().unwrap();
        re.find(&branch)
            .map(|regex_match| format!("({}): ", regex_match.as_str()))
            .unwrap_or_else(|| ": ".to_string())
    } else {
        ": ".to_string()
    };

    let user_input = Text::new("Enter commit message:").prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;
    let message = format!("{}{}{}", commit_type, ticket, user_input);

    let should_commit = Confirm::new(&format!("Commit with message: \"{}\"?", message))
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    git_operations::add_files(selected_files, &mut index)
        .map_err(|e| format!("Failed to add files: {}", e))?;
    
    if should_commit {
        git_operations::commit_and_push(repo, index, message)
            .map_err(|e| format!("❌ Commit and push failed: {}", e))?;
        
        println!("✅ Commit and push successful!");
    } else {
        println!("❌ Commit canceled or failed to get user confirmation.");
    }

    Ok(())
}