use super::diagram_text_emitter;
use super::epic_info;

fn prelude() -> String {
    return format!("<html>\n");
}

fn head() -> String {
    return r#"
  <head>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script>
      mermaid.initialize({ startOnLoad: true });
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
    let diagram_postlude = r#"
        </div>
      </div>
        "#;
    let side_panel = r#"
      <div class="side-panel">
        <div>Story name<div>
        <div>Estimate<div>
        <div>Labels<div>
        <div>Blockers<div>
        <div>Story description<div>
      </div>
        "#;
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
