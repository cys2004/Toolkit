/// A representation of an article type in the FOSScope repository rule, which defines the types of articles that can be found in the repository.
///
/// # Fields
/// - `article_type`(`type` in TOML file): The type of the article. e.g. `news`, `tech`.
/// - `description`: The description of the article type. e.g. `News Articles`, `Tech Articles`.
/// - `directory`: The directory where the article type is stored. e.g. `{step}/news`, `{step}/tech`.
///
/// Check the [related design documentation](https://github.com/FOSScope/Toolkit/blob/main/docs/dev/design/repo-rule.md)
/// and [RepoRule](struct.RepoRule.html) definition for more information.
#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
pub struct Article {
    /// The type of the article.
    #[serde(rename = "type")]
    pub article_type: String,
    /// The description of the article type.
    pub description: String,
    /// The directory where the article type is stored.
    pub directory: String,
}

impl Article {
    pub fn new(article_type: String, description: String, directory: String) -> Self {
        Self {
            article_type,
            description,
            directory,
        }
    }
}

/// A representation of an action in the FOSScope repository rule, which actions that can be performed on the repository.
///
/// # Fields
/// - `action`: The action name. e.g. `select`, `translate`, `review`.
/// - `description`: The description of the action. e.g. `Select an article to translate`.
/// - `command`: The command that should be executed when the action is performed. e.g. `TOUCH source/{article}.md`.
///
/// Check the [related design documentation](https://github.com/FOSScope/Toolkit/blob/main/docs/dev/design/repo-rule.md)
/// and [RepoRule](struct.RepoRule.html) definition for more information.
#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
pub struct Action {
    /// The name of the action.
    pub action: String,
    /// The description of the action.
    pub description: String,
    /// The command that should be executed when the action is performed.
    pub command: String,
}

impl Action {
    pub fn new(action: String, description: String, command: String) -> Self {
        Self {
            action,
            description,
            command,
        }
    }
}

/// A representation of the Git rule in the FOSScope repository rule, which defines how repositories should be managed.
///
/// # Fields
/// - `branch_naming`: The naming convention for branches. Which is a string containing placeholders that will be replaced with the actual values.
/// - `commit_message`: The commit message template. Which is a string containing placeholders that will be replaced with the actual values.
///
/// # Example
/// - `branch_naming`: `{action}/{type}/{article}`
/// - `commit_message`: `[{action.desc}][{type.desc}]: {article.title}`
///
/// Check the [related design documentation](https://github.com/FOSScope/Toolkit/blob/main/docs/dev/design/repo-rule.md)
/// and [RepoRule](struct.RepoRule.html) definition for more information.
#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
pub struct GitRule {
    /// The naming convention for branches. Has to be formatted with real values.
    pub branch_naming: String,
    /// The commit message template. Has to be formatted with real values.
    pub commit_message: String,
}

impl GitRule {
    pub fn new(branch_naming: String, commit_message: String) -> Self {
        Self {
            branch_naming,
            commit_message,
        }
    }
}

/// A representation of the FOSScope repository rule, which defines how repositories should be managed.
///
/// The rule includes a list of articles, a list of actions, and a Git rule.
///
/// # Fields
/// - `articles`([Article](struct.Article.html)): A list of types of articles that can be found in the repository.
/// - `actions`([Action](struct.Action.html)): : A list of actions that can be performed on the repository.
/// - `git`([GitRule](struct.GitRule.html)): The Git rule that defines how the repository should be managed.
///
/// Check the [related design documentation](https://github.com/FOSScope/Toolkit/blob/main/docs/dev/design/repo-rule.md) for more information.
#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
pub struct RepoRule {
    /// The list of types of articles that can be found in the repository.
    pub articles: Vec<Article>,
    /// The list of actions that can be performed on the repository.
    pub actions: Vec<Action>,
    /// The Git rule that defines how the repository should be managed.
    pub git: GitRule,
}

impl RepoRule {
    pub fn new(articles: Vec<Article>, actions: Vec<Action>, git: GitRule) -> Self {
        Self {
            articles,
            actions,
            git,
        }
    }
}