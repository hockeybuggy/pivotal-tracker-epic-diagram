mod epic_info {
    use serde::{Deserialize, Serialize};

    const BASE_URL: &str = "https://www.pivotaltracker.com/services/v5";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Epic {
        pub id: u64,
        pub project_id: u64,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Story {
        pub id: u64,
        pub project_id: u64,
        pub name: String,
    }

    pub async fn get_epics_with_label(
        label: &str,
    ) -> Result<Vec<Epic>, Box<dyn std::error::Error>> {
        let tracker_token = std::env::var("PIVOTAL_TRACKER_TOKEN").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let epics: Vec<Epic> = reqwest::Client::new()
            .get(&format!(
                "{}/projects/{}/epics?filter={}",
                BASE_URL, project_id, label
            ))
            .header("X-TrackerToken", tracker_token)
            .send()
            .await?
            .json()
            .await?;
        println!("{:?}", epics);

        Ok(epics)
    }

    pub async fn get_stories_for_epic(
        epic_label: &str,
    ) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
        let tracker_token = std::env::var("PIVOTAL_TRACKER_TOKEN").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let stories: Vec<Story> = reqwest::Client::new()
            .get(&format!(
                "{}/projects/{}/stories?with_label={}",
                BASE_URL, project_id, epic_label
            ))
            .header("X-TrackerToken", tracker_token)
            .send()
            .await?
            .json()
            .await?;
        println!("{:?}", stories);

        Ok(stories)
    }
}

mod diagram_text_emitter {
    use super::epic_info;

    pub fn dot_representation(epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
        let story_ids: Vec<String> = stories
            .into_iter()
            .map(|story| format!("{}", story.id))
            .collect();
        format!("{}\nstories: {:?}", epic.name, story_ids)
    }
}

use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("Fetching epic from Tracker...");
    let epic_label = std::env::var("EPIC_LABEL").unwrap();
    let epics = epic_info::get_epics_with_label(&epic_label).await?;

    // This fetch now seems to mostly function as a "does this epic exist" check.
    if epics.len() == 0 {
        panic!("Error: Could not find epic matching label.")
    } else if epics.len() > 1 {
        panic!("Error: Found more than one epic matching label.")
    }

    let stories = epic_info::get_stories_for_epic(&epic_label).await?;

    let dot_diagram: String = diagram_text_emitter::dot_representation(&epics[0], &stories);

    println!("Diagram: {}", dot_diagram);

    // TODO generate represntation in a textual format (dot? mermaidjs? plantuml?)
    Ok(())
}
