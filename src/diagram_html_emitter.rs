use super::diagram_text_emitter;
use super::epic_info;

fn story_details(story: &epic_info::Story) -> String {
    return format!(
        "\
            <div id='story-details-{story_id}' class='not-selected'>\
                <p>id: <a href='{story_url}' target=_blank>{story_id}</a></p>\
                <p>name: {name}</p>\
            </div>\
            \n",
        story_id = &story.id,
        story_url = &story.url,
        name = &story.name,
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
    body, main {
      height: 100%;
    }

    body {
      display: flex;
      flex-direction: column;
    }

    main {
      display: flex;
      flex-direction: row;
    }

    nav {
      display: flex;
      flex-direction: row;
      border: solid 1px black;
    }

    .diagram-container {
      border: solid 1px black;
      flex-grow: 1;
    }
    .side-panel {
      border: solid 1px black;
      width: 33%;
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

fn nav() -> String {
    return r#"
    <nav>
      <h1>NAME OF EPIC</h1>
    </nav>
    "#
    .to_string();
}

fn main(epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
    let dot_diagram: String = diagram_text_emitter::dot_representation(&epic, &stories);
    let diagram_prelude = r#"
      <div class="diagram-container">
        <div class="mermaid">
        "#;
    // let diagram_prelude = r#"
    //   <div class="diagram-container">
    //     <textarea class="mermaid" rows="40", cols="80">
    //     "#;
    // let diagram_postlude = r#"
    //     </textarea>
    //   </div>
    //     "#;
    let diagram_postlude = r#"
        </div>
      </div>
        "#;
    let story_details_nodes: String = stories.into_iter().map(story_details).collect();
    let side_panel = format!(
        r#"
      <div class="side-panel">
        {}
      </div>
        "#,
        story_details_nodes
    );
    return format!(
        "<main>{}\n{}\n{}\n{}\n</main>",
        diagram_prelude, dot_diagram, diagram_postlude, side_panel
    );
}

pub fn generate_page(epic: &epic_info::Epic, stories: &Vec<epic_info::Story>) -> String {
    return format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        prelude(),
        head(),
        body_prelude(),
        nav(),
        main(&epic, &stories),
        body_postlude(),
        postlude()
    );
}
