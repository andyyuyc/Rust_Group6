use git2::{Repository, MergeOptions, FetchOptions, RemoteCallbacks, Cred};
use std::io;
use std::path::Path;

fn pull_changes(remote_repo_path: &str, local_branch: &str) -> io::Result<()> {
    let repo = Repository::open(Path::new("."))?;
    let mut remote = repo.find_remote(remote_repo_path)?;

    let mut fo = FetchOptions::new();
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, Path::new("~/.ssh/id_rsa"), None)
    });
    fo.remote_callbacks(callbacks);

    remote.fetch(&[local_branch], Some(&mut fo), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.is_up_to_date() {
        println!("Already up to date!");
    } else if analysis.is_fast_forward() {
        let refname = format!("refs/heads/{}", local_branch);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(None)?;
    } else {
        println!("Non-fast-forward updates are not supported in this example.");
    }

    Ok(())
}

fn push_changes(local_repo_path: &str, remote_branch: &str) -> io::Result<()> {
    let repo = Repository::open(Path::new(local_repo_path))?;
    let mut remote = repo.find_remote("origin")?;

    let mut push_options = git2::PushOptions::new();
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, Path::new("~/.ssh/id_rsa"), None)
    });
    push_options.remote_callbacks(callbacks);

    remote.push(&[&format!("refs/heads/{}:refs/heads/{}", local_repo_path, remote_branch)], Some(&mut push_options))?;

    Ok(())
}