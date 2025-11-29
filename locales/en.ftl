# OEIS TUI - English Translations

# Application
app-title = OEIS TUI
app-subtitle = On-Line Encyclopedia of Integer Sequences

# Greeting Screen
greeting-title = Welcome to OEIS TUI
greeting-line1 = A beautiful terminal interface for exploring integer sequences
greeting-line2 = Press 'i' or '/' to start searching
greeting-line3 = Press 'r' for a random sequence
greeting-line4 = Press 'w' for webcam mode
greeting-line5 = Press 'Ctrl+H' for help
greeting-copyright = © OEIS Foundation Inc. - All sequence data is property of OEIS
greeting-version = Version 0.1.0

# Welcome / empty states
welcome-title = Welcome to OEIS TUI
welcome-subtitle = The On-Line Encyclopedia of Integer Sequences (unofficial TUI)
welcome-prompt = Enter a sequence, word, or A-number to begin.
welcome-search-label = Search OEIS
welcome-enter-hint = Enter to search
welcome-esc-hint = Esc to dismiss
welcome-hero-subtitle = Find known integer sequences, discover references, and explore relationships.
welcome-hero-tips = Try: 1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = Press 'i' or '/' at any time to jump into search.
search-empty-title = No search results yet
search-tips-title = Search tips:
search-tip-terms = • Enter sequence terms: 1,1,2,3,5,8,13
search-tip-anumber = • Search by A-number: id:A000045
search-tip-keyword = • Search by keyword: fibonacci
search-tip-prefixes = • Use prefixes: keyword:nice author:Sloane
search-start-hint = Press 'i' or '/' to start searching
search-recently-viewed = Recently Viewed
search-history-empty = No history yet
search-bookmarks-title = Bookmarks
search-bookmarks-empty = No bookmarks yet. Press 'b' in detail view to bookmark sequences.
search-bookmarks-loading = Loading...
search-bookmarks-notes = Notes
search-results-title = Results

# Search Screen
search-title = Search OEIS
search-input-label = Search
search-input-placeholder = Enter sequence terms (e.g., 1,2,3,5,8,13), A-number, or keyword...
search-status-results = { $count ->
    [0] No results found
    [one] 1 result found
    *[other] { $count } results found
}
search-status-page = Page { $current } of { $total }
search-status-loading = Searching...
search-status-fetching = Please wait while we fetch results from OEIS
search-status-error = Error: { $message }
search-no-results = No results found
search-result-one = 1 result found
search-result-many = { $count } results found
search-result-many-plus = { $count }+ results found
search-table-anumber = A-Number
search-table-name = Name
search-table-data = Data Preview
search-table-views = Views
search-block-results = Results
search-block-preview = Preview
search-block-details = Details
search-preview-empty = No preview available
search-invalid-tab = Invalid tab
search-view-count = { $count ->
    [one] 1 view
    *[other] { $count } views
}
search-help = i,/ Search | ↑↓ Navigate | ←→ Page | Enter View | p Preview | r Random | w Webcam | s Settings | Ctrl+H Help | q Quit
search-help-search = Search
search-help-navigate = Navigate
search-help-page = Page
search-help-view = View
search-help-preview = Preview
search-help-bookmarks = Bookmarks
search-help-random = Random
search-help-webcam = Webcam
search-help-settings = Settings
search-help-help = Help
search-help-quit = Quit
search-help-click = Select
search-help-click-x2 = Open
search-help-scroll = Move

# Detail View
detail-tab-overview = Overview
detail-tab-formulas = Formulas
detail-tab-code = Code
detail-tab-references = References
detail-tab-crossrefs = Cross-refs
detail-tab-metadata = Metadata
detail-tab-graph = Graph
detail-tab-export = Export
detail-offset = Offset
detail-keywords = Keywords
detail-author = Author
detail-created = Created
detail-modified = Last Modified
detail-comments = Comments
detail-data = Sequence Data
detail-formulas = Formulas
detail-examples = Examples
detail-maple = Maple Code
detail-mathematica = Mathematica Code
detail-programs = Other Programs
detail-references = References
detail-links = Links
detail-crossrefs = Cross-references
detail-extensions = Extensions
detail-no-data = No data available
detail-no-sequence = No sequence loaded
detail-block-sequence = Sequence
detail-block-details = Details
detail-section-data = Data
detail-section-comments = Comments
detail-section-examples = Examples
detail-help = Tab Switch | ↑↓ Scroll | g Graph | e Export | o Browser | b Bookmark | Esc Back
detail-help-next-link = Next link
detail-help-prev-link = Previous link
detail-help-switch-tab = Switch tab
detail-help-follow-link = Follow link
detail-help-scroll = Scroll
detail-help-graph = Graph
detail-help-export = Export
detail-help-browser = Open in browser
detail-help-bookmark = Bookmark
detail-bookmarked = Bookmarked
detail-not-bookmarked = Not Bookmarked
detail-help-bfile = Fetch B-file
detail-help-more = More
detail-help-modal-title = Detail View - Keyboard Shortcuts
detail-bfile-available = Extended data available
detail-bfile-fetch = Press 'f' to fetch B-file
detail-bfile-loading = Loading B-file...
detail-bfile-loaded = ✓ Loaded {$count} terms
detail-bfile-error = B-file unavailable
detail-bfile-not-found = B-file not found for this sequence

# Graph View
graph-title = Graph View
graph-line = Line Chart
graph-scatter = Scatter Plot
graph-log = Logarithmic Scatter Plot
graph-pin = Pin Plot
graph-no-data = No numeric data to plot
graph-no-positive = No positive values to plot on logarithmic scale
graph-current = Current
graph-help = 1 Line | 2 Scatter | 3 Log Scatter | 4 Pin Plot | Esc Back
graph-help-line = Line
graph-help-scatter = Scatter
graph-help-log = Log Scatter
graph-help-pin = Pin Plot
graph-help-back = Back to detail view

# Export Screen
export-title = Export Sequence
export-format = Select Format
export-json = JSON
export-json-desc = Full sequence data with all metadata
export-csv = CSV
export-csv-desc = Sequence values in comma-separated format
export-txt = TXT
export-txt-desc = Human-readable plain text format
export-markdown = Markdown
export-markdown-desc = Formatted documentation
export-preview = Preview
export-no-sequence = No sequence to export
export-success = Exported to clipboard successfully
export-file-success = Saved to file: { $path }
export-error = Export failed: { $message }
export-help = ↑↓ Select | 1-5 Quick Select | Enter Clipboard | Ctrl+S Save | Esc Cancel
export-bfile = B-file
export-bfile-desc = Extended sequence data (index value pairs)
export-bfile-not-loaded = B-file not loaded - press 'f' in detail view
export-select-format = Select Format
export-cancel = Cancel

# Export Content Labels
export-label-offset = Offset
export-label-keywords = Keywords
export-label-data = Data
export-label-author = Author
export-label-created = Created
export-label-modified = Last Modified
export-label-references = References
export-label-revision = Revision

# Export Section Headers
export-section-sequence-data = Sequence Data
export-section-metadata = Metadata
export-section-comments = Comments
export-section-formulas = Formulas
export-section-examples = Examples
export-section-code = Code
export-section-references = References
export-section-links = Links
export-section-crossrefs = Cross-references

# Export Subsection Headers
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = Other Programs

# Export Format Specific
export-csv-header = A-number,Name,Values
export-markdown-source = Source
export-markdown-oeis-credit = Data from the On-Line Encyclopedia of Integer Sequences (OEIS)

# Webcam Mode
webcam-title = OEIS Webcam - Sequence Browser
webcam-category = Category
webcam-category-all = All Sequences
webcam-category-all-desc = Browse all OEIS sequences
webcam-category-best = Best Sequences
webcam-category-best-desc = Interesting and notable sequences (keyword:nice)
webcam-category-needing = Needing Terms
webcam-category-needing-desc = Sequences requesting more terms (keyword:more)
webcam-category-recent = Recent Additions
webcam-category-recent-desc = Recently added sequences (keyword:new)
webcam-interval = Refresh Interval
webcam-interval-manual = Manual
webcam-interval-manual-desc = Press Space to advance
webcam-interval-5s = 5 seconds
webcam-interval-5s-desc = Auto-refresh every 5s
webcam-interval-10s = 10 seconds
webcam-interval-10s-desc = Auto-refresh every 10s
webcam-interval-20s = 20 seconds
webcam-interval-20s-desc = Auto-refresh every 20s
webcam-interval-30s = 30 seconds
webcam-interval-30s-desc = Auto-refresh every 30s
webcam-interval-1m = 1 minute
webcam-interval-1m-desc = Auto-refresh every 60s
webcam-current-sequence = Current Sequence
webcam-no-sequence = No sequence loaded
webcam-load-first = Press Space or Enter to load first sequence
webcam-refresh-in = Next refresh in { $seconds } seconds...
webcam-more-comments = ... and { $count } more comments
webcam-sequence-offset = Offset
webcam-sequence-keywords = Keywords
webcam-sequence-data-title = Sequence Data
webcam-sequence-comments-title = Comments
webcam-help = Space/Enter Next | ←→ Category | ↑↓ Interval | 0-5 Quick | d Detail | Esc Back
webcam-help-next = Next
webcam-help-category = Category
webcam-help-interval = Interval
webcam-help-quick = Quick Interval
webcam-help-detail = Detail View
webcam-help-back = Back

# Settings Screen
settings-title = Settings
settings-block-settings = Settings
settings-language = Language
settings-language-desc = Select interface language
settings-theme = Theme
settings-theme-desc = Color scheme (coming soon)
settings-cache = Cache
settings-cache-desc = Manage local cache
settings-cache-clear = Clear Cache
settings-cache-size = Cache size: { $size }
settings-block-themes = Themes
settings-block-animation = Welcome Animation
settings-help = ↑↓ Navigate | Enter Select | Esc Back
settings-help-switch = Switch Section
settings-help-navigate = Navigate
settings-help-apply = Apply
settings-help-cycle-theme = Cycle Theme
settings-help-back = Back

# About Screen
about-title = About OEIS TUI
about-version = Version
about-author = Created by
about-license = License
about-built-with = Built with
about-links = Links
about-repository = Repository
about-oeis-link = OEIS Website
about-disclaimer = This is an unofficial client and is not affiliated with or endorsed by The OEIS Foundation Inc.

# Help Screen
help-title = Help - Keyboard Shortcuts
help-global = Global Controls
help-global-quit = Quit application
help-global-help = Show/hide help
help-global-back = Go back / Cancel
help-search = Search Screen
help-search-input = Start searching
help-search-navigate = Navigate results
help-search-page = Previous/Next page
help-search-view = View selected sequence
help-search-random = Random sequence
help-search-preview = Toggle preview pane
help-search-preview-tabs = Switch preview tab
help-search-mouse-select = Click to select result
help-search-mouse-open = Double-click to open result
help-search-mouse-scroll = Scroll wheel to move preview/results
help-search-webcam = Webcam mode
help-detail = Detail View
help-detail-links = Cycle highlighted link
help-detail-tabs = Switch tabs
help-detail-open-link = Open highlighted link
help-detail-scroll = Scroll content
help-detail-scroll-fast = Scroll faster
help-detail-graph = View graph
help-detail-export = Export sequence
help-detail-browser = Open in browser
help-detail-bookmark = Toggle bookmark
help-graph = Graph View
help-graph-types = Switch graph type
help-export = Export Screen
help-export-select = Select format
help-export-quick = Quick format selection
help-export-clipboard = Export to clipboard
help-export-file = Save to file
help-webcam = Webcam Mode
help-webcam-next = Load next sequence
help-webcam-category = Switch category
help-webcam-interval = Change refresh interval
help-webcam-quick = Quick interval selection
help-webcam-detail = Jump to detail view

# Common
common-loading = Loading...
common-error = Error
common-success = Success
common-cancel = Cancel
common-ok = OK
common-yes = Yes
common-no = No
common-back = Back
common-next = Next
common-previous = Previous
common-page = Page
common-of = of

# Errors
error-network = Network error: Unable to connect to OEIS
error-api = API error: { $message }
error-parse = Parse error: Invalid data format
error-cache = Cache error: { $message }
error-clipboard = Clipboard error: { $message }
error-file = File error: { $message }
error-unknown = Unknown error occurred
