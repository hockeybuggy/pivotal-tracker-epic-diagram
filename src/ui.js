var currentStoryId = null;
var currentNodeRectangle = null;

function hideEmptyState() {
  let dom = document.getElementById("empty-state");
  dom.className = "not-selected";
}

function showEmptyState() {
  let dom = document.getElementById("empty-state");
  dom.className = "selected";
}

function hideStory(storyId) {
  let dom = document.getElementById("story-details-" + storyId);
  dom.className = "not-selected";
}

function showStory(storyId) {
  let dom = document.getElementById("story-details-" + storyId);
  dom.className = "selected";
}

function highlightSelectedNode(node) {
  node.id = "highlight-this-node";
}

function unHighlightSelectedNode(node) {
  node.id = "unhighlight-this-node";
}

function storyNodeClickedCallback(storyId, nodeRectangle) {
  if (currentStoryId) {
    hideStory(currentStoryId);
    if (currentNodeRectangle) {
      unHighlightSelectedNode(currentNodeRectangle);
    }
  } else {
    hideEmptyState();
  }

  if (currentStoryId !== storyId) {
    // If selecting a new story show it
    showStory(storyId);
    highlightSelectedNode(nodeRectangle);
    currentStoryId = storyId;
    currentNodeRectangle = nodeRectangle;
  } else {
    // If the story is already selected de-select it.
    showEmptyState();
    currentStoryId = null;
    currentNodeRectangle = null;
  }
}

window.addEventListener("load", function () {
  var svgs = d3.selectAll(".mermaid svg");
  svgs.each(function () {
    var svg = d3.select(this);

    // Bind click events to each of the story nodes. This will get the story id
    // and the `rect` element within the `svg` this will then be used to select
    // the story.
    svg.selectAll(".nodes").on("click", (e) => {
      let storyId;
      let rectangle;
      if (e.target.tagName === "rect") {
        rectangle = e.target;
        storyId = e.target.nextSibling.textContent;
      } else {
        rectangle = e.target.parentNode.parentNode.parentNode.previousSibling;
        storyId = e.target.textContent;
      }
      storyNodeClickedCallback(storyId, rectangle);
    });

    var inner = svg.select("g");
    var zoom = d3
      .zoom()
      .scaleExtent([1 / 2, 8])
      .on("zoom", function (event) {
        inner.attr("transform", event.transform);
      });
    svg.call(zoom).on("dblclick.zoom", null);
  });
});
