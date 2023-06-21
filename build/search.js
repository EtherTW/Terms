$(document).ready(function(){
  $("#searchInput").on("keyup", function() {
    applyFilters();
  });
});

function applyFilters() {
  var searchValue = $("#searchInput").val().toLowerCase();
  var tagFilter = window.tagFilter;  // Current tag filter from tags.js
  
  $("#termsTable tr").each(function() {
    var row = $(this);
    var matchesSearch = row.text().toLowerCase().indexOf(searchValue) > -1;
    var matchesTag = tagFilter == null || row.text().indexOf(tagFilter) > -1;
    row.toggleClass('filtered', matchesSearch && matchesTag);
  });
}