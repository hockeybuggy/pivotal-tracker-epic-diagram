extern crate clap;
extern crate sanitize_filename;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, ArgGroup, Command};
use dotenv;

mod diagram_html_emitter;
mod diagram_text_emitter;
mod epic_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let matches = Command::new("main")
        .about("Creates a visual diagram of a Pivotal Tracker epic")
        .arg(
            Arg::new("epic_name")
                .short('e')
                .long("epic")
                .help("The epic name in Pivotal Tracker")
                .takes_value(true)
        )
        .arg(
            Arg::new("epic_id")
                .short('i')
                .long("epic_id")
                .help("The epic ID in Pivotal Tracker")
                .takes_value(true)
        )
        .group(
            ArgGroup::new("epic_details")
                .args(&["epic_name", "epic_id"])
                .multiple(false)
                .required(true)
        )
        .get_matches();

    println!("Fetching epic from Tracker...");

    let mut epics = vec![];
    let epic_label = matches.value_of("epic_name");
    let epic_id = matches.value_of("epic_id");

    if matches.is_present("epic_name") {
        epics = epic_info::get_epics_with_label(epic_label.unwrap()).await?;
    } else if matches.is_present("epic_id") {
        epics = vec![epic_info::get_epic_with_id(epic_id.unwrap()).await?];
    }

    // This fetch now seems to mostly function as a "does this epic exist" check.
    if epics.len() == 0 {
        panic!("Error: Could not find epic matching label.")
    } else if epics.len() > 1 {
        panic!("Error: Found more than one epic matching label.")
    }

    println!("Fetching stories from Tracker...");
    let mut stories = epic_info::get_stories_with_label(&epics[0].name).await?;

    println!("Fetching blockers for each story...");
    for mut story in &mut stories {
        story.blockers = Some(epic_info::get_blockers_for_story_id(&story.id).await?);
        story.labels = Some(epic_info::get_labels_for_story_id(&story.id).await?);
    }

    let page: String = diagram_html_emitter::generate_page(&epics[0], &stories);

    let file_name = sanitize_filename::sanitize(format!(
        "epic_diagram-{}-{}.html", epics[0].id.to_string(), epics[0].name
    ));
    let path = Path::new(&file_name);
    let display = path.display();
    println!("Writing Diagram to file...");

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(page.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    return Ok(());
}
