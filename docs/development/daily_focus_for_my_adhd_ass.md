## Daily Focus for ADHD

### Summary of To-Do List

#### High Priority Tasks

1. **Get App Back in Working Order**
   - Setup 'hide equation labels' state and add `onChange` events to webviews.
   - Fix tabs resizing issue.
   - Get component JavaScript working.
   - Add event listener for `cdrm-codeblock-copy` and emit notification.
   - Fix Bold & italic text parser.
   - Handle overflow of tab by toggling CSS class based on height.
   - Revert component glue files to initial setup using `conundrum/ts` package.
   - Add katex CSS to mdx container.

2. **Remainder of Today**
   - Handle new images on blog post.
   - Fix bug requiring details page refresh on Mac.
   - Fix new note first display issue.
   - Handle rendering code to HTML.
   - Move `from_props` methods to accept state param for parsing/compilation phases.
   - Move all katex font methods to use binary stored data.
   - Fix Grid component issue with integer parser.
   - Fix Blockquote bug with multi-line entries.
   - Write JavaScript to copy code from code block and emit event.
   - Fix inline underline component breaking out of paragraph.

3. **UI/UX Improvements**
   - Reattach theme selector to Swift for Mac and iOS.
   - Fix independent markdown view page rendering.
   - Write script to generate single JavaScript/TypeScript file from component files.
   - Handle missing parsers (Line Item with checkbox, Plain line item, Numbered line item, Full table).

4. **AI Integration**
   - Create 'triggers' for AI-generated content (e.g., `FlusterAI.generateStudyGuide()`, `FlusterAI.createFlashCards()`).
   - Make sure AI triggers can be set to auto, confirm, or never modes.

5. **Documentation & Website**
   - Update docs to reflect upcoming Apple release.
   - Fix admonition padding issue on website and main app.

6. **Performance & Offline**
   - Move regex queries to `HashMap<String, bool>` in Rust.
   - Fix issue with italic field in title syntax.
   - Relay detailed errors to user in webview.
   - Create `subtle` or `underline` property for `Tabs` component.
   - Handle `Icon` component dynamically loading icons.

7. **Release Checklist**
   - Ensure all tests pass (Swift, Typescript, Rust).
   - Review Apple's nomination process for featured app.

#### Next Steps

- Work on integrating typst support for offline use.
- Setup seeding of notes on desktop app.
- Move remaining components (e.g., `AINoteSummary`).
- Handle missing parsers (CodeBlock, Footnote, Table, Strikethrough, Escapable math blocks, etc.).
- Fix bug with bibliography entries not being associated with notes.
- Ensure AI summarization works and handle light mode for `AISummaryContainer`.
- Handle force regeneration of AI summaries.
- Work on React property parsers for better AI integration.

#### Bug Fixes

- Fix issue with bibliography entries not being associated with notes.
- Fix 'paper' button creating 2 pages on desktop.
- Ensure note can be loaded when iPad is in portrait view.
- Fix `NoteDetailSheet` requiring refresh on Mac.
- Handle `Icon` component dynamically loading icons.
- Ensure iPad app sends `ErrorStateReset` event after manual save.

#### Documentation

- Update card component documentation.
- Fix admonition padding issue on website and main app.
- Update docs to reflect upcoming Apple release.

#### Offline & Performance

- Move regex queries to `HashMap<String, bool>` in Rust.
- Fix issue with italic field in title syntax.
- Relay detailed errors to user in webview.
- Create `subtle` or `underline` property for `Tabs` component.
- Handle `Icon` component dynamically loading icons.
- Ensure iPad app sends `ErrorStateReset` event after manual save.

#### AI & Release

- Create 'triggers' for AI-generated content (e.g., `FlusterAI.generateStudyGuide()`, `FlusterAI.createFlashCards()`).
- Make sure AI triggers can be set to auto, confirm, or never modes.
- Ensure AI summarization works and handle light mode for `AISummaryContainer`.
- Handle force regeneration of AI summaries.
- Work on React property parsers for better AI integration.

#### Final Notes

- Ensure all tests pass (Swift, Typescript, Rust) before release.
- Review Apple's nomination process for featured app.
- Make sure AI triggers can be set to auto, confirm, or never modes.
- Ensure AI summarization works and handle light mode for `AISummaryContainer`.
- Handle force regeneration of AI summaries.
- Work on React property parsers for better AI integration.

---

### Release Timeline

- **April 30, 2026**: Finalize all tasks and prepare for release.
- **May 7, 2026**: Release the app, ensuring all features are working as expected.
- **May 14, 2026**: Monitor user feedback and address any issues promptly.

---

### Summary

This to-do list covers all high-priority tasks needed to get the app back in working order, fix bugs, improve UI/UX, integrate AI, and prepare for release. The goal is to release the app within the next 2 weeks, ensuring all features are functional and user-friendly.