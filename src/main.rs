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
    pub enum StoryState {
        #[serde(alias = "accepted")]
        Accepted,
        #[serde(alias = "delieved")]
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
        pub blocked_story_ids: Vec<u64>,
        pub blockers: Option<Vec<Blocker>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    pub struct Blocker {
        pub id: u64,
        pub story_id: u64,
    }

    async fn request_project(
        path: String,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let tracker_token = std::env::var("PIVOTAL_TRACKER_TOKEN").unwrap();
        let project_id = std::env::var("PROJECT_ID").unwrap();
        let response = reqwest::Client::new()
            .get(&format!("{}/projects/{}/{}", BASE_URL, project_id, path))
            .header("X-TrackerToken", tracker_token)
            .send()
            .await?;
        return Ok(response);
    }

    pub async fn get_epics_with_label(
        label: &str,
    ) -> Result<Vec<Epic>, Box<dyn std::error::Error>> {
        let response = request_project(format!("/epics?filter={}", label)).await?;
        let epics: Vec<Epic> = response.json().await?;
        return Ok(epics);
    }

    pub async fn get_stories_with_label(
        epic_label: &str,
    ) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
        let response = request_project(format!(
            "stories?with_label={epic_label}&fields=:default,blocked_story_ids",
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
}

mod diagram_text_emitter {
    use super::epic_info;

    const GREY: &str = "GREY";
    const BLUE: &str = "BLUE";
    const YELLOW: &str = "YELLOW";
    const GREEN: &str = "GREEN";
    const RED: &str = "RED";

    fn prelude() -> String {
        // Colours based on this page: https://www.pivotaltracker.com/help/articles/story_states/
        let colour_classes: String = [
            (GREY, "#e0e2e5", "#c4c5c5"),
            (BLUE, "#507bbd", "#2959a4"),
            (YELLOW, "#f5b04f", "#fc9d17"),
            (GREEN, "#94c37f", "#5fa640"),
            (RED, "#e87450", "#ec4d22"),
        ]
        .iter()
        .map(|(name, fill, stroke)| {
            format!(
                "\tclassDef {name} fill:{fill},stroke:{stroke};\n",
                name = name,
                fill = fill,
                stroke = stroke,
            )
        })
        .collect::<Vec<String>>()
        .join("");

        return format!(
            "\
            graph TD\n\
            {colour_classes}\
            ",
            colour_classes = colour_classes
        );
        // TODO more colours
    }

    fn story_node(story: &epic_info::Story) -> String {
        let node_id = format!("{}", &story.id);
        let status = match &story.current_state {
            &epic_info::StoryState::Accepted => format!(":::{}", &GREEN),
            &epic_info::StoryState::Delivered => format!(":::{}", &GREEN),
            &epic_info::StoryState::Finished => format!(":::{}", &GREEN),
            &epic_info::StoryState::Started => format!(":::{}", &BLUE),
            &epic_info::StoryState::Rejected => format!(":::{}", &RED),
            &epic_info::StoryState::Planned => format!(":::{}", &GREY),
            &epic_info::StoryState::Unstarted => format!(":::{}", &GREY),
            &epic_info::StoryState::Unscheduled => format!(":::{}", &GREY),
        };
        // Double quotes will prevent the file from parsing
        let safe_name = &story.name.replace("\"", "'");
        let link = format!("click {} \"{}\" \"{}\"", &story.id, &story.url, &safe_name);
        let deps = match &story.blockers {
            Some(blockers) => blockers
                .iter()
                .map(|blocker| {
                    format!(
                        "\t{from} --> {to}\n",
                        from = &story.id,
                        to = blocker.story_id
                    )
                })
                .collect::<Vec<String>>()
                .join(""),
            None => "".to_owned(),
        };

        return format!(
            "\
            \t{node_id}{status}\n\
            \t{link}\n\
            {deps}\
            \n",
            node_id = node_id,
            status = status,
            link = link,
            deps = deps,
        );
    }

    pub fn dot_representation(_epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
        let story_nodes: String = stories.into_iter().map(story_node).collect();
        return format!("{}\n{}", prelude(), story_nodes);
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    println!("Fetching stories from Tracker...");
    let mut stories = epic_info::get_stories_with_label(&epic_label).await?;

    println!("Fetching blockers for each story...");
    for mut story in &mut stories {
        story.blockers = Some(epic_info::get_blockers_for_story_id(&story.id).await?);
    }

    let dot_diagram: String = diagram_text_emitter::dot_representation(&epics[0], &stories);

    let path = Path::new("epic_diagram.mmd");
    let display = path.display();
    println!("Writing Diagram to file...");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(dot_diagram.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    return Ok(());
}
