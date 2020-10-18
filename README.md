
# Pivotal tracker epic diagrams

This repo creates a diagram of tickets and their blockers for a particular epic.

## Configuration

This program is configured using the `.env` file. This file will load
environment variables the program starts.

- `PIVOTAL_TRACKER_TOKEN` - API token for Pivotal tracker
- `PROJECT_ID` - The project id that contains the epic.
- `EPIC_LABEL` - The label of the Epic you want to draw


## Dependencies

This program is written in the Rust programming language. This can be installed
with the tool `rustup`. This repo also assumes that you use `nvm` to manage node versions.

## To generate diagrams

```
nvm use
npm install
./generate
```
