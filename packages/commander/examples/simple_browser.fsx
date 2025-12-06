// Simple File Browser Example
// Demonstrates basic file browsing functionality

use commander::*;

// Initialize the file browser
let browser = FileBrowser.new();

// Set starting directory (defaults to current directory)
browser.setPath(".");

// Configure display options
browser.setShowHidden(false);
browser.setSortBy("name");  // Options: "name", "size", "modified"

// Get directory listing
let entries = browser.list();

print("Current directory: " + browser.getCurrentPath());
print("Total entries: " + toString(entries.length));
print("");

// Display entries
entries.forEach((entry) => {
  let icon = if entry.isDirectory then "[DIR]" else "[FILE]";
  let size = if entry.isDirectory then "" else " (" + formatSize(entry.size) + ")";
  print(icon + " " + entry.name + size);
});

// Helper function to format file sizes
fn formatSize(bytes: Int) -> String {
  if bytes < 1024 {
    toString(bytes) + " B"
  } else if bytes < 1024 * 1024 {
    toString(bytes / 1024) + " KB"
  } else if bytes < 1024 * 1024 * 1024 {
    toString(bytes / (1024 * 1024)) + " MB"
  } else {
    toString(bytes / (1024 * 1024 * 1024)) + " GB"
  }
}

// Filter by extension
print("\nMarkdown files:");
let mdFiles = browser.filter((entry) => {
  !entry.isDirectory && entry.name.endsWith(".md")
});

mdFiles.forEach((entry) => {
  print("  " + entry.name);
});

// Search for files
print("\nSearch results for 'test':");
let searchResults = browser.search("test");

searchResults.forEach((entry) => {
  print("  " + entry.path);
});
