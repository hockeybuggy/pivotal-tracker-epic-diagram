#!/bin/bash

MMDC="node_modules/.bin/mmdc"

echo "creating mermaid.js file.."
cargo run

echo "generating image diagram.."
$MMDC \
    --input epic_diagram.mmd \
    --output epic_diagram.png \
    --configFile mermaid-config.json

echo "generating svg diagram.."
$MMDC \
    --input epic_diagram.mmd \
    --output epic_diagram.svg \
    --configFile mermaid-config.json

echo "Done!"
