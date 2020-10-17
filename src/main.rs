mod epic_info {
    use serde::{Deserialize, Serialize};

    const BASE_URL: &str = "https://www.pivotaltracker.com/services/v5";
    const PROJECT_ID: &str = "2452598"; // TODO extract

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Epic {
        pub id: u64,
        pub project_id: u64,
        pub name: String,
    }

    pub async fn get_epics_with_label(
        label: &str,
    ) -> Result<Vec<Epic>, Box<dyn std::error::Error>> {
        let tracker_token = std::env::var("PIVOTAL_TRACKER_TOKEN").unwrap();
        let epics: Vec<Epic> = reqwest::Client::new()
            .get(&format!(
                "{}/projects/{}/epics?filter={}",
                BASE_URL, PROJECT_ID, label
            ))
            .header("X-TrackerToken", tracker_token)
            .send()
            .await?
            .json()
            .await?;
        println!("{:?}", epics);

        Ok(epics)
    }
}

mod diagram_text_emitter {
    use super::epic_info;

    pub fn dot_representation(epic: &epic_info::Epic) -> String {
        format!("{}", epic.name)
    }
}

use dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("Fetching epic from Tracker...");
    let epics = epic_info::get_epics_with_label("PLF: Hubspot Sync").await?;

    if epics.len() == 0 {
        panic!("Error: Could not find epic matching label.")
    } else if epics.len() > 1 {
        panic!("Error: Found more than one epic matching label.")
    }

    let dot_diagram: String = diagram_text_emitter::dot_representation(&epics[0]);

    println!("Diagram: {}", dot_diagram);

    // TODO generate represntation in a textual format (dot? mermaidjs? plantuml?)
    Ok(())
}
