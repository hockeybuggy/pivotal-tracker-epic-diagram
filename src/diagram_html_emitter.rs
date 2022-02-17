use chrono::{DateTime, Local, SecondsFormat};
use std::fs;
use super::diagram_text_emitter;
use super::epic_info;

fn format_labels(labels: &Option<Vec<epic_info::Label>>) -> String {
    let label_names = match labels {
        Some(labels) => labels
            .iter()
            .map(|l| {
                format!("<span>{}</span>", l.name)
            })
            .collect::<Vec<String>>()
            .join(", "),
        None => "".to_owned(),
    };

    return label_names;
}

fn story_details(story: &epic_info::Story) -> String {
    let label_names = format_labels(&story.labels);
    return format!(
        "\
            <div id='story-details-{story_id}' class='not-selected'>\
                <p>id: <a href='{story_url}' target=_blank>{story_id}</a></p>\
                <p>name: {name}</p>\
                <p>labels: {labels}</p>\
                <p>current state: {current_state:?}</p>\
            </div>\
            \n",
        story_id = &story.id,
        story_url = &story.url,
        name = &story.name,
        labels = label_names,
        current_state = &story.current_state,
    );
}

fn prelude() -> String {
    return format!("<!DOCTYPE html>\n<html lang='en-CA'>\n");
}

fn head() -> String {
    let css = fs::read_to_string("src/styles.css").unwrap();
    return format!(r#"
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">

    <script>
        var currentNode = null;

        function hideEmptyState() {{
            let dom = document.getElementById("empty-state");
            dom.className = "not-selected";
        }}

        function hideStory(storyId) {{
            let dom = document.getElementById("story-details-" + storyId);
            dom.className = "not-selected";
        }}

        function showStory(storyId) {{
            let dom = document.getElementById("story-details-" + storyId);
            dom.className = "selected";
        }}

        function ticketNodeCallback(storyId) {{
            if (currentNode) {{
                hideStory(currentNode);
            }} else {{
                hideEmptyState();
            }}
            showStory(storyId);
            currentNode = storyId;
        }}
    </script>

    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script>
      mermaid.initialize({{
          startOnLoad: true,
          securityLevel: 'loose',
      }});
    </script>
  <style>
      {}
  </style>
  </head>
  "#, css);
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
    let story_details_nodes: String = stories.into_iter().map(story_details).collect();
    let panel = format!(
        r#"
      <div class="panel">
        <div id="empty-state">
         <p>Click a story to see it's details.</p>
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
