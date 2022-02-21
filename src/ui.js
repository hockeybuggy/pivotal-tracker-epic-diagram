var currentNode = null;

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

function ticketNodeClickedCallback(storyId) {
  if (currentNode) {
    hideStory(currentNode);
  } else {
    hideEmptyState();
  }
  if (currentNode !== storyId) {
    // If selecting a new story show it
    showStory(storyId);
    currentNode = storyId;
  } else {
    // If the story is already selected de-select it.
    showEmptyState();
    currentNode = null;
  }
}

window.addEventListener("load", function () {
  var svgs = d3.selectAll(".mermaid svg");
  svgs.each(function () {
    var svg = d3.select(this);

    // TODO check if this is needed
    // Wrap the element in an extra `g`
    svg.html("<g>" + svg.html() + "</g>");

    // Bind new events
    // TODO add styling to the selected element
    // TODO make it clear that the nodes are clickable
    svg.selectAll(".nodes").on("click", (e) => {
      let storyId;
      if (e.target.tagName === "rect") {
        storyId = e.target.nextSibling.textContent;
      } else {
        storyId = e.target.textContent;
      }
      ticketNodeClickedCallback(storyId);
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
