use chrono::{DateTime, Local, SecondsFormat};

use super::diagram_text_emitter;
use super::epic_info;

fn story_details(story: &epic_info::Story) -> String {
    return format!(
        "\
            <div id='story-details-{story_id}' class='not-selected'>\
                <p>id: <a href='{story_url}' target=_blank>{story_id}</a></p>\
                <p>name: {name}</p>\
                <p>current state: {current_state:?}</p>\
            </div>\
            \n",
        story_id = &story.id,
        story_url = &story.url,
        name = &story.name,
        current_state = &story.current_state,
    );
}

fn prelude() -> String {
    return format!("<!DOCTYPE html>\n<html lang='en-CA'>\n");
}

fn head() -> String {
    return r#"
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">

    <script>
        var currentNode = null;

        function hideEmptyState() {
            let dom = document.getElementById("empty-state");
            dom.className = "not-selected";
        }

        function hideStory(storyId) {
            let dom = document.getElementById("story-details-" + storyId);
            dom.className = "not-selected";
        }

        function showStory(storyId) {
            let dom = document.getElementById("story-details-" + storyId);
            dom.className = "selected";
        }

        function ticketNodeCallback(storyId) {
            if (currentNode) {
                hideStory(currentNode);
            } else {
                hideEmptyState();
            }
            showStory(storyId);
            currentNode = storyId;
        }
    </script>

    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script>
      mermaid.initialize({
          startOnLoad: true,
          securityLevel: 'loose',
      });
    </script>

  <style>
    html, body, main {
      height: 100%;
    }

    body {
      display: flex;
      flex-direction: column;
    }

    main {
      display: flex;
      flex-direction: column-reverse;
    }

    nav {
      display: flex;
      flex-direction: row;
      border: solid 1px black;
    }
    nav h1 {
      margin: 0.25rem 1rem;
    }

    .diagram-container {
      border: solid 1px black;
      flex-grow: 1;
    }
    .panel {
      padding: 0.5rem;
      border: solid 1px black;
      border-top: none;
      border-bottom: none;
      min-height: 125px;
    }

    .not-selected {
      display: none;
    }
    .selected {
      display: block;
    }

  </style>
  </head>
  "#
    .to_string();
}

fn postlude() -> String {
    return format!("</html>\n");
}

fn body_prelude() -> String {
    return format!("<body>\n");
}

fn body_postlude() -> String {
    return format!("</body>\n");
}

fn nav(epic: &epic_info::Epic) -> String {
    return format!(
        "\
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
        diagram_prelude, dot_diagram, diagram_postlude, panel
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
