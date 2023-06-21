const colors = ["secondary", "success", "danger", "warning", "primary", "info", "light", "dark"];

/**
 * setup colorMap[tag] => color
 * @param {*} tags 
 * @returns 
 */
function createColorMap(tags) {
  let colorMap = {};
  for (let i = 0; i < tags.length; i++) {
    colorMap[tags[i]] = colors[i % colors.length];
  }
  return colorMap;
}

$(document).ready(function() {
  let tags = ["mev", "zk", "defi", "protocol", "layer2"];
  let colorMap = createColorMap(tags);
  // Now you can use colorMap to set the colors of your badges
  $(".badge").each(function() {
    let tag = $(this).text();
    $(this).addClass("badge-" + colorMap[tag]);
  });

  // Save reference to all table rows
  let rows = $("#termsTable tbody tr");

  // Apply colors to tags
  $(document).ready(function() {
      $(".badge").each(function() {
          let tag = $(this).text();
          $(this).addClass("badge-" + colorMap[tag]);
      });

      // Make tags clickable
      $(".badge").click(function() {
        window.tagFilter = $(this).text();
        applyFilters();
      });

  });

  // Reset filtered tag button functionality
  $("#resetButton").click(function() {
    window.tagFilter = null;
    $("#searchInput").val("");
    applyFilters();
  });

  // Initially, all rows are filtered
  rows.addClass('filtered');
});

