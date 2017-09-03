use std::collections::HashMap;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub repository: Repository,
    pub subject: NotificationSubject,

    pub reason: String,

    pub unread: bool,
    pub updated_at: Option<String>,
    pub last_read_at: Option<String>,
    pub url: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub owner: User,
    pub name: String,
    pub full_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CodeOfConduct>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    pub html_url: String,
    pub clone_url: Option<String>,
    pub git_url: Option<String>,
    pub mirror_url: Option<String>,
    pub ssh_url: Option<String>,
    pub svn_url: Option<String>,

    pub language: Option<String>,
    pub fork: bool,
    pub forks_count: Option<i64>,
    pub network_count: Option<i64>,
    pub open_issues_count: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub subscribers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub size: Option<i64>,
    pub auto_init: Option<bool>,

    pub parent: Option<Box<Repository>>,
    pub source: Option<Box<Repository>>,
    pub organization: Option<Organization>,
    pub permissions: Option<HashMap<String, bool>>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub topics: Option<Vec<String>>,

    pub license: Option<License>,

    pub private: bool,
    pub has_issues: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_downloads: Option<bool>,
    pub license_template: Option<String>,
    pub gitignore_template: Option<String>,

    pub team_id: Option<i64>,

    // API URLs
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub trees_url: String,
    pub teams_url: String,

    pub text_matches: Option<Vec<TextMatch>>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: i64,
    // Todo: Implement continuing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub html_url: String,
    pub name: String,
    // Todo: Implement continuing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub url: String,

    pub spdx_id: String,
    pub html_url: String,
    // Todo: Implement continuing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct CodeOfConduct {
    pub name: String,
    pub key: String,
    pub url: String,
    pub body: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TextMatch {
    pub object_url: String,
    pub object_type: String,
    pub property: String,
    pub fragment: String,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Match {
    pub text: String,
    pub indices: Vec<i64>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NotificationSubject {
    pub title: String,
    pub url: String,
    pub latest_comment_url: String,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TimeStamp {
    timestamp: String,
}
