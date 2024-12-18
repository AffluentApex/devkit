use git2::Repository;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{path::Path, thread, time::Duration};

pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    pub fn analyze(&self) -> Result<GitStats, git2::Error> {
        let mut stats = GitStats::default();
        let m = MultiProgress::new();
        
        // Setup progress bars with emojis
        let pb_commits = m.add(ProgressBar::new_spinner());
        pb_commits.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} 📚 {msg}")
                .unwrap(),
        );
        
        let pb_branches = m.add(ProgressBar::new_spinner());
        pb_branches.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} 🌿 {msg}")
                .unwrap(),
        );

        // Analyze commits
        pb_commits.set_message("Analyzing commit history...");
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        stats.total_commits = revwalk.count();
        thread::sleep(Duration::from_millis(500)); // Add slight delay for effect
        pb_commits.finish_with_message(format!("Found {} commits! 🎉", stats.total_commits));

        // Analyze branches
        pb_branches.set_message("Counting branches...");
        stats.branch_count = self.repo.branches(None)?.count();
        thread::sleep(Duration::from_millis(500)); // Add slight delay for effect
        pb_branches.finish_with_message(format!("Found {} branches! 🌳", stats.branch_count));

        // Get latest commit info
        if let Ok(head) = self.repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                stats.latest_commit = Some(CommitInfo {
                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                    message: commit.message().unwrap_or("No message").to_string(),
                    time: commit.time().seconds(),
                });
            }
        }

        Ok(stats)
    }
}

#[derive(Default)]
pub struct GitStats {
    pub total_commits: usize,
    pub branch_count: usize,
    pub latest_commit: Option<CommitInfo>,
}

pub struct CommitInfo {
    pub author: String,
    pub message: String,
    pub time: i64,
}
