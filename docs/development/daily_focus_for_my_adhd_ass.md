# Daily Focus for My ADHD-Ass

## Prioritized Tasks to Release Fluster in 2 Weeks

### Critical Fixes
- Fix new note first display issue
- Handle rendering code to HTML
- Reimplement `window.setBibtexEditorContent`
- Fix auto-inserted code block and title-underscore parsing
- Ensure summary can be accepted or declined from UI

### Core Component Moves
- Move `from_props` methods to accept state param
- Move all KateX font methods to use binary stored data
- Move over remaining components: AINoteSummary, Table of Contents

### Parser Development
- Implement CodeBlock parser with meta string check
- Develop Footnote, Table, Strikethrough, Escapable math blocks, and Escapable strings parsers
- Reimplement Paragraphs and Line Item with various checked states
- Support `[My link](myId@10:30:00)` timestamp link syntax

### AI Integration
- Create `FlusterAI.generateStudyGuide()` and `FlusterAI.createFlashCards()`
- Implement AI trigger modes: auto, confirm, never
- Develop React property parsers for better AI input

### Desktop App
- Work on AI summarization and syncing method for file system notes
- Fix scroll restoration in landscape mode

### iPad App
- Ensure note loads properly in portrait view
- Implement `ErrorStateReset` event for manual save requests

### Webviews
- Add `cursor-pointer` class to all taggables
- Handle error display in webview

### Documentation
- Update docs to reflect Apple release
- Fix admonition padding issue

### Offline Features
- Fix italic field syntax in title
- Create searchable lazy icon list
- Verify iPad app functionality with error state resets

### Performance
- Move regex queries to `HashMap<String, bool>` for faster lookup

### Release Prep
- Ensure all tests pass (Swift, Typescript, Rust)
- Review Apple nomination process for featured status
- Handle light mode for `AISummaryContainer`
- Implement force regeneration for AI summaries
- Add `output_format` property for conditional rendering

### Additional Tasks
- Create quick plot generator for conundrum parsing time
- Fix `NoteDetailSheet` refresh issues on Mac
- Develop `SiaString` class for string compression/comparison
- Apply note front matter summary to new summary class
- Handle new line items on initial load
- Implement global search page on Mac
- Set AI availability states and adjust `AiContainerPhase1...` component
- Write tests for all `-ignoreParser` flags

## Notes
- Focus on high-impact tasks first
- Break down complex tasks into smaller, manageable pieces
- Prioritize tasks that directly impact user experience
- Keep daily goals focused and achievable
- Review progress daily and adjust priorities as needed

Let's tackle these one by one and keep the momentum going!