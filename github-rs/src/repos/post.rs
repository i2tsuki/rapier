//! Access the Repos portion of the GitHub API
imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    Sha
    Statuses
    Repo
    Repos
    Owner
);

from!(
    @Sha
        => Executor
    @PostQueryBuilder
        -> Repos = "repos"
    @Repos
        => Owner
    @Owner
        => Repo
    @Repo
        -> Statuses = "statuses"
    @Statuses
        => Sha
);

impl_macro!(
    @Sha
        |
        |-> execute
    @Repos
        |
        |=> owner ->  Owner = username_str
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> statuses -> Statuses
        |
    @Statuses
        |
        |=> sha -> Sha = sha_str
);
