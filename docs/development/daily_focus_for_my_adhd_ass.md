# Daily Focus for My ADHD Brain

## Priority Tasks

### 1. **Move Over Components**
- Move over all KateX font methods to use binary stored data.
- Move over `from_props` methods to accept a state param to break up parsing and compilation phases.
- Reimplement `window.setBibtexEditorContent`.
- Move over `AINoteSummary` component.

### 2. **Fix Critical Issues**
- Fix new note first display issue.
- Fix code block title-underscore parsing issue.
- Handle auto-inserted code block.
- Fix issue with bibliography entries not being associated with note properly.
- Handle rest of summary UI.
- Make sure summary can be accepted or declined from the UI appropriately.

### 3. **Parser Improvements**
- Work on integrating typst support (major value add).
- Setup seeding of notes on desktop app.
- Work on remaining parsers (CodeBlock, Footnote, Table, Strikethrough, Escapable math blocks, Escapable strings, Paragraphs, Line Item, Timestamp link syntax).

### 4. **UI/UX Enhancements**
- Handle scroll restoration on desktop app while in landscape mode.
- Make sure all taggables have a 'cursor-pointer' class.
- Handle light mode of `AISummaryContainer` card.
- Handle force regeneration of AI summaries.
- Handle React property parsers for better component parsing.

### 5. **Offline & Performance**
- Fix issue with italic field in title syntax.
- Move regex queries in `get_component_map` to `HashMap<String, bool>` in Rust.
- Handle `Icon` component that dynamically loads icons.
- Create searchable lazy list of icons.
- Verify iPad app is working again with error state reset and initial state function.

### 6. **Release Preparation**
- Ensure all tests are passing (Swift, Typescript, Rust).
- Review Apple's nomination process for featured app.
- Handle AI availability states and generate note summaries.
- Write tests for all `-ignoreParser` flags.

## Next Steps
- Focus on the most critical tasks first to ensure a stable release within the next 2 weeks.
- Break down larger tasks into smaller, manageable chunks.
- Prioritize tasks that have the most impact on user experience and app functionality.

## Notes
- Keep the focus on high-impact tasks to ensure a smooth release.
- Regularly review progress and adjust priorities as needed.
- Stay organized and maintain a clear plan to stay on track for the release date.

