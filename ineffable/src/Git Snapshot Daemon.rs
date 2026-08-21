use git2::{Repository, IndexAddOption};
use std::{thread, time};

fn main() -> Result<(), git2::Error> {
    let repo = Repository::open("ledger_repo")?;
    loop {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let signature = repo.signature()?;
        let parent = repo.head()?.peel_to_commit()?;
        let message = format!("Ledger snapshot at {:?}", chrono::Utc::now());
        repo.commit(Some("HEAD"), &signature, &signature, &message, &tree, &[&parent])?;
        println!("✅ Snapshot committed and pushed.");
        thread::sleep(time::Duration::from_secs(3600));
    }
}