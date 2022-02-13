use lazy_static::lazy_static;
use regex::Regex;

use super::epic_info;

const GREY: &str = "GREY";
const BLUE: &str = "BLUE";
const YELLOW: &str = "YELLOW";
const GREEN: &str = "GREEN";
const RED: &str = "RED";

fn prelude() -> String {
    // Colours based on this page: https://www.pivotaltracker.com/help/articles/story_states/
    let colour_classes: String = [
        (GREY, "#e0e2e5", "#c4c5c5", "#000"),
        (BLUE, "#507bbd", "#2959a4", "#fff"),
        (YELLOW, "#f5b04f", "#fc9d17", "#fff"),
        (GREEN, "#94c37f", "#5fa640", "#fff"),
        (RED, "#e87450", "#ec4d22", "#fff"),
    ]
    .iter()
    .map(|(name, fill, stroke, colour)| {
        format!(
            "\tclassDef {name} fill:{fill},stroke:{stroke},color:{colour};\n",
            name = name,
            fill = fill,
            stroke = stroke,
            colour = colour,
        )
    })
    .collect::<Vec<String>>()
    .join("");

    return format!(
        "\
            graph TD\n\
            {colour_classes}\n\
            ",
        colour_classes = colour_classes
    );
}

fn get_ticket_numbers_from_blocker_description(blocker_desc: &str) -> Vec<String> {
    lazy_static! {
        static ref SHORT_TAG_REGEX: Regex = Regex::new(r"\#([0-9]+)").unwrap();
        static ref FULL_URL_REGEX: Regex =
            Regex::new(r"https://www.pivotaltracker.com/story/show/([0-9]+)").unwrap();
        static ref ALTERNATE_URL_REGEX: Regex =
            Regex::new(r"https://www.pivotaltracker.com/n/projects/([0-9]+)/stories/([0-9]+)").unwrap();
    }
    let mut tickets: Vec<String> = Vec::new();
    let mut short_tag_ticket_ids = SHORT_TAG_REGEX
        .captures_iter(blocker_desc)
        .map(|cap| cap.get(1).map_or("".to_owned(), |m| m.as_str().to_owned()))
        .collect::<Vec<String>>();
    tickets.append(&mut short_tag_ticket_ids);
    let mut full_url_ticket_ids = FULL_URL_REGEX
        .captures_iter(blocker_desc)
        .map(|cap| cap.get(1).map_or("".to_owned(), |m| m.as_str().to_owned()))
        .collect::<Vec<String>>();
    tickets.append(&mut full_url_ticket_ids);
    let mut alternate_full_url_ticket_ids = ALTERNATE_URL_REGEX
        .captures_iter(blocker_desc)
        .map(|cap| cap.get(2).map_or("".to_owned(), |m| m.as_str().to_owned()))
        .collect::<Vec<String>>();
    tickets.append(&mut alternate_full_url_ticket_ids);
    return tickets;
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
    // let safe_name = &story.name.replace("\"", "'");
    // let link = format!(
    //     "click {} \"{}\" \"{}\" _blank",
    //     &story.id, &story.url, &safe_name
    // );
    let link = format!("click {} call ticketNodeCallback()", &story.id);

    let deps = match &story.blockers {
        Some(blockers) => blockers
            .iter()
            .map(|blocker| {
                let blocking_tickets =
                    get_ticket_numbers_from_blocker_description(&blocker.description);
                blocking_tickets
                    .iter()
                    .map(|blocking_ticket_id| {
                        format!(
                            "\t{to} --> {from}\n",
                            to = blocking_ticket_id,
                            from = &story.id,
                        )
                    })
                    .collect::<Vec<String>>()
            })
            .flatten()
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
