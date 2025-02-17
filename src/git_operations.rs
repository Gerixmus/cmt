use std::{path::Path, process::Command};
use git2::{Repository, StatusOptions};

pub fn get_repository() -> Option<Repository> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("This is not a Git repository.");
            return None;
        }
    };
    Some(repo)
}

pub fn get_untracked(repo: &Repository) -> Vec<String> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return Vec::new();
        }
    };

    let mut files_to_add = Vec::new();
    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_wt_new() || status.is_wt_modified() {
            if let Some(path) = entry.path() {
                files_to_add.push(path.to_string());
            }
        }
    }

    files_to_add
}

pub fn add_files(selected_files: Vec<String>, index: &mut git2::Index) -> Result<(), git2::Error>{
    for file in selected_files.iter() {
        let path = Path::new(file);
        if let Err(err) = index.add_path(path) {
            return Err(err);
        }
    }
    
    index.write()?;
    Ok(())
}

pub fn push_to_origin() -> Result<(), String> {
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("HEAD") 
        .output()
        .expect("Failed to execute git push command");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{}", error_message));
    }

    Ok(())
}

pub fn commit_and_push(repo: git2::Repository, mut index: git2::Index, message: String) -> Result<(), String> {
    let signature = repo.signature().unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();
    let head = repo.head();
    let parent_commits = match head {
        Ok(head) => {
            if let Ok(parent) = head.peel_to_commit() {
                vec![parent]
            } else {
                vec![]
            }
        }
        Err(_) => vec![],
    };
    let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();
    if let Err(err) = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &parent_refs,
    ) {
        return Err(format!("❌ Commit failed: {}", err));
    }
    if let Err(err) = push_to_origin() {
        return Err(format!("❌ Push failed: {}", err));
    }
    
    Ok(())
}
