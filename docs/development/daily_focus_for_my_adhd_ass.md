# Daily Focus for ADHD Assistant

## Priority Tasks

### Critical Fixes
- Fix new note first display issue
- Handle rendering code to HTML
- Move over all KateX font methods to use binary stored data
- Fix issue with bibliography entries not being associated with note properly
- Handle breakout of paragraph when only components are nested
- Fix Grid component integer parser issue
- Handle Blockquote bug with multi-line entries
- Implement JavaScript to copy code from code block
- Reimplement `window.setBibtexEditorContent`

### High Priority
- Work on integrating typst support (major value add)
- Setup seeding of notes on desktop app
- Move over remaining components (AINoteSummary, Table of Contents)
- Work on remaining parsers (CodeBlock, Footnote, Table, Strikethrough, Escapable math blocks, Escap, Paragraphs, Line Item, Timestamp link syntax)

### Medium Priority
- Handle scroll restoration on desktop app in landscape mode
- Work on syncing method to create notes from file system
- Make sure note can be loaded when iPad is in portrait view
- Handle React property parsers for AI components
- Create `subtle` or `underline` property for Tabs component
- Handle `Icon` component that dynamically loads icons
- Verify iPad app is working again
- Handle color scheme note changing editor theme
- Create quick plot generator for mean conundrum parsing time
- Start setting AI availability states

### Low Priority
- Update docs to reflect upcoming Apple release
- Fix issue with italic field in title syntax
- Handle force regeneration of AI summaries
- Handle light mode for AISummaryContainer card
- Create searchable lazy list of icons
- Handle bug with new line items not working on initial new line
- Handle Global search page on Mac
- Fix issue with note being set as modified just by viewing it in Mac app

## Release Preparation
- Ensure all tests are passing (Swift, Typescript, Rust)
- Review Apple's nomination process for featured app
- Handle AI summary regeneration and R3 vector storage
- Write tests for all `-ignoreParser` flags

## Additional Notes
- Focus on critical fixes first to ensure stability
- Prioritize tasks that have the most impact on user experience
- Break down complex tasks into smaller, manageable chunks
- Use the release checklist as a guide for final preparations
- Keep the AI summary and R3 vector generation as high priority for release readiness

This focus list is designed to help maintain productivity and ensure the app is ready for release within the next 2 weeks. Remember to take breaks and stay focused on the most important tasks first.