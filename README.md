# Pivotal Tracker epic diagrams

This repo creates a diagram of stories and their blockers for a particular Epic
in Pivotal Tracker.


## How does this work

This project makes calls to Pivotal tracker's API to fetch all the stories and
their blockers. Once these have been retrieved we find the story ids within
the blocker's descriptions. We use that to generate a
[Mermaid.js](https://mermaid-js.github.io/mermaid/#/) based diagram hosted in
an html page. This page will show basic details of the story when the
corresponding node in the diagram is selected.

![Example png diagram](./examples/diagram_v2.png?t=0217202201729)


## Configuration

This program is configured using the `.env` file. This file will load
environment variables the program starts.

- `PIVOTAL_TRACKER_TOKEN` - API token for Pivotal tracker
- `PROJECT_ID` - The project id that contains the epic.

If you're getting started you can run:

```sh
cp .env .env.sample
```


## Dependencies

This program is written in the Rust programming language. This can be installed
with the tool `rustup`.


## To generate diagram page

### Command arguments

| Argument | Description | Example |
| --- | --- | --- |
| `--epic` | The label of the Epic you want to draw | `--epic "project gutenberg"` |

### Example command line
The output page can be generated with:

```
cargo run -- --epic "<your epic name>"
```


## Possible improvements

- [ ] We could save some API calls by fetching the blocker ids when we get the
      list of stories and not make API calls for stories with no blocker ids.
- [ ] Make the project a SPA that makes its own requests and generates its own pages.
