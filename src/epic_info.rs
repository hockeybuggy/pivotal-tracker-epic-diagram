use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://www.pivotaltracker.com/services/v5";

#[derive(Serialize, Deserialize, Debug)]
pub struct Epic {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StoryState {
    #[serde(alias = "accepted")]
    Accepted,
    #[serde(alias = "delivered")]
    Delivered,
    #[serde(alias = "finished")]
    Finished,
    #[serde(alias = "started")]
    Started,
    #[serde(alias = "rejected")]
    Rejected,
    #[serde(alias = "planned")]
    Planned,
    #[serde(alias = "unstarted")]
    Unstarted,
    #[serde(alias = "unscheduled")]
    Unscheduled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub url: String,
    pub current_state: StoryState,
    pub blockers: Option<Vec<Blocker>>,
    pub labels: Option<Vec<Label>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blocker {
    pub id: u64,
    pub story_id: u64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub id: u64,
    pub kind: String,
    pub name: String,
}

async fn request_project(path: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let tracker_token = std::env::var("PIVOTAL_TRACKER_TOKEN").unwrap();
    let project_id = std::env::var("PROJECT_ID").unwrap();
    let response = reqwest::Client::new()
        .get(&format!("{}/projects/{}/{}", BASE_URL, project_id, path))
        .header("X-TrackerToken", tracker_token)
        .send()
        .await?;
    return Ok(response);
}

pub async fn get_epics_with_label(label: &str) -> Result<Vec<Epic>, Box<dyn std::error::Error>> {
    let response = request_project(format!("/epics?filter={}", label)).await?;
    let epics: Vec<Epic> = response.json().await?;
    return Ok(epics);
}

pub async fn get_stories_with_label(
    epic_label: &str,
) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let response = request_project(format!(
        "stories?with_label={epic_label}",
        epic_label = epic_label,
    ))
    .await?;
    let stories: Vec<Story> = response.json().await?;
    return Ok(stories);
}

pub async fn get_blockers_for_story_id(
    story_id: &u64,
) -> Result<Vec<Blocker>, Box<dyn std::error::Error>> {
    let response =
        request_project(format!("stories/{story_id}/blockers", story_id = story_id)).await?;
    let blockers: Vec<Blocker> = response.json().await?;
    return Ok(blockers);
}

pub async fn get_labels_for_story_id(
    story_id: &u64,
) -> Result<Vec<Label>, Box<dyn std::error::Error>> {
    let response =
        request_project(format!("stories/{story_id}/labels", story_id = story_id)).await?;
    let labels: Vec<Label> = response.json().await?;
    return Ok(labels);
}
