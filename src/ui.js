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

window.addEventListener("load", function () {
  var svgs = d3.selectAll(".mermaid svg");
  svgs.each(function () {
    var svg = d3.select(this);
    svg.html("<g>" + svg.html() + "</g>");
    var inner = svg.select("g");
    var zoom = d3
      .zoom()
      .on("zoom", function (event) {
        inner.attr("transform", event.transform);
      })
      .clickDistance(20);
    svg.call(zoom);
  });
});
