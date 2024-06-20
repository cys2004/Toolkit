use octocrab::Octocrab;
use super::super::models::GitHubRepo;

pub struct GitHubApi {
    username: String,
    octocrab: Octocrab,
}

impl GitHubApi {
    pub fn new(username: String, octocrab: Octocrab) -> Self {
        Self {
            username,
            octocrab,
        }
    }

    /**
     * Get a list of all forks of the given repository.
     */
    pub async fn get_forks(&self, upstream: GitHubRepo) -> Vec<GitHubRepo> {
        let mut forks = Vec::new();
        let mut page: u32 = 1;
        loop {
            let response = self.octocrab.repos(
                upstream.owner.clone(), upstream.name.clone()
            ).list_forks().page(page).per_page(100).send().await.unwrap();

            // Yes, this is a bit of a hacky way...
            // If you know a better way to do this, please let me know (or submit a PR)!
            // - Cubik
            let next_page = response.next.clone();

            // Only include the owner and name of the forked repository.
            forks.extend(response.into_iter().map(
                |repo| GitHubRepo::new(repo.owner.unwrap().login, repo.name)
            ));

            // If there's no next page, break out of the loop.
            if next_page.is_none() {
                break
            }

            page += 1;
        }
        forks
    }

    /**
     * Get the fork of upstream repository that belongs to the currently signed in GitHub user.
     */
    pub async fn get_user_fork(&self, upstream: GitHubRepo) -> Result<GitHubRepo, &str> {
        let forks = self.get_forks(upstream).await;
        for fork in forks {
            if fork.owner == self.username {
                return Ok(fork);
            }
        }
        Err("User has not forked the upstream repository.")
    }

    /**
     * Get the username (login) of the currently signed-in GitHub user.
     */
    async fn get_user(&self) -> String {
        self.octocrab.current().user().await.unwrap().login
    }

    /**
     * Verify that the username of the currently signed in GitHub user matches the username
     * provided to the GitHubApi.
     */
    pub async fn verify_user(&self) -> bool {
        self.username == self.get_user().await
    }

    /**
     * Validate that the given repository exists and is a fork of the upstream repository.
     */
    pub async fn validate_repo(&self, repo: GitHubRepo, upstream: GitHubRepo) -> Result<(), &str> {
        if let Ok(repo) = self.octocrab.repos(repo.owner, repo.name).get().await {
            if repo.fork.unwrap_or(false) {
                if repo.parent.unwrap().full_name.unwrap() == upstream.get_full_name() {
                    Ok(())
                } else {
                    Err("Repository is not a fork of the upstream repository")
                }
            } else {
                Err("Repository is not a fork")
            }
        } else {
            Err("Repository does not exist")
        }
    }
}
