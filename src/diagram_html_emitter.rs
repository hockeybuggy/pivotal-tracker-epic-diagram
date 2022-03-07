use super::diagram_text_emitter;
use super::epic_info;
use chrono::{DateTime, Local, SecondsFormat};
use std::fs;

fn format_story_labels(story_labels: &Option<Vec<epic_info::Label>>, epic_label: &str) -> String {
    let epic_lowercase = epic_label.to_lowercase();
    let labels_html = match story_labels {
        Some(story_labels) => story_labels
            .into_iter()
            .filter(|l| l.name != epic_lowercase)
            .map(|l| format!("<span class=\"badge-label\">{}</span>", l.name))
            .collect::<Vec<String>>()
            .join(", "),
        None => "".to_owned(),
    };

    let mut story_labels_html = format!("<span class=\"badge-epic\">{}</span>", epic_lowercase);
    if labels_html.len() > 0 {
        story_labels_html = format!("{}, {}", &story_labels_html, &labels_html);
    }

    return story_labels_html;
}

fn story_details(story: &epic_info::Story, epic_name: &str) -> String {
    let labels_html = format_story_labels(&story.labels, epic_name);
    return format!(
        "\
            <div id='story-details-{story_id}' class='not-selected'>\
                <p><b>Id:</b> <a href='{story_url}' target=_blank>{story_id}</a></p>\
                <p><b>Name:</b> {name}</p>\
                <p><b>Labels:</b> {labels}</p>\
                <p><b>Current State:</b> {current_state:?}</p>\
            </div>\
            \n",
        story_id = &story.id,
        story_url = &story.url,
        name = &story.name,
        labels = labels_html,
        current_state = &story.current_state,
    );
}

fn prelude() -> String {
    return format!("<!DOCTYPE html>\n<html lang='en-CA'>\n");
}

fn head() -> String {
    let ui_script = fs::read_to_string("src/ui.js").unwrap();
    let css = fs::read_to_string("src/styles.css").unwrap();
    return format!(
        r#"
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">

    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script src="https://d3js.org/d3.v6.min.js"></script>

    <script>
      mermaid.initialize({{
        startOnLoad: true,
      }});
    </script>

    <script>
      {ui_script}
    </script>

    <style>
      {css}
    </style>
  </head>
  "#,
        css = css,
        ui_script = ui_script,
    );
}

fn postlude() -> String {
    return format!("</html>\n");
}

fn body_prelude() -> String {
    return format!("<body>\n");
}

fn body_postlude() -> String {
    return format!("\n</body>\n");
}

fn nav(epic: &epic_info::Epic) -> String {
    return format!(
        "\
        <div class='wrapper'>\
        <nav>\
          <h1>Epic: {}</h1>\
        </nav>\
    ",
        &epic.name,
    );
}

fn footer() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.to_rfc3339_opts(SecondsFormat::Secs, true);
    return format!(
        "\
        </div>\
        <footer>\
          <p>generated on: {}</p>\
        </footer>\
    ",
        formatted_date,
    );
}

fn main(epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
    let dot_diagram: String = diagram_text_emitter::dot_representation(&epic, &stories);
    let diagram_prelude = r#"
      <div class="diagram-container">
        <div class="mermaid">
        "#;
    let diagram_postlude = r#"
        </div>
      </div>
        "#;
    let story_details_nodes: String = stories
        .into_iter()
        .map(|sd| story_details(sd, &epic.name))
        .collect();
    let panel = format!(
        r#"
      <div class="panel">
        <div id="empty-state">
         <p>Click a story to see its details.</p>
        </div>
        {}
      </div>
        "#,
        story_details_nodes
    );
    return format!(
        "<main>{}\n{}\n{}\n{}\n</main>",
        panel, diagram_prelude, dot_diagram, diagram_postlude
    );
}

pub fn generate_page(epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
    return format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        prelude(),
        head(),
        body_prelude(),
        nav(&epic),
        main(&epic, &stories),
        footer(),
        body_postlude(),
        postlude()
    );
}
