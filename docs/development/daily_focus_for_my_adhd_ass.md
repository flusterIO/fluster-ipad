## Daily Focus for My ADHD Brain

### Summary of To-Do List (Generated: Monday, May 4th, 2026 — 6:40 PM CST)

#### High Priority Tasks

1. **Resume work on Table Parsing**
   - Complete the TDD setup for the markdown table parser.
   - Run the test: `cargo nextest run parsers::markdown::table::markdown_table_heading_row::tests::parses_basic_table_heading_row --no-capture`

2. **Rewrite Paper**
   - Focus on the new derivation and validate the Solar mass equivalence against Earth data.

3. **Fix Markdown View Page**
   - Ensure tabs render properly on load.

4. **Implement Missing Parsers**
   - List Item with checkbox (various checked states)
   - Numbered line item
   - Full table

5. **Reimplement `window.setBibtexEditorContent`**
   - Ensure it works as a buffer operation.

6. **Handle Summary UI**
   - Ensure summary can be accepted or declined from the UI.

#### Pre-Release Milestones

- **Dictionary Entry Page**
   - Render dictionary entries to HTML
   - Handle rendering in React from the Swift DB

- **Documentation Pages**
   - Parse & Render Tables
   - Parse & Render Lists (Unordered, Ordered, Checkbox)
   - Parse & Render Footnotes
   - Emoji search and other doc components

- **Components to Move**
   - AINoteSummary
   - Table of Contents (TOC)

#### Up Next

- **Integrate Typst Support**
   - Major value add for offline use

- **Setup Seeding of Notes**
   - For the desktop app release

- **Work on Remaining Parsers**

#### Missing or Incomplete Parsers

- CodeBlock (Check for meta string without `--`)
- Footnote
- Table (GFM if performance allows)
- Strikethrough text
- Escapable math blocks
- Escapable strings
- Line Item (various checked states)
- Timestamp link syntax

#### Bug Log

- Fix bibliography entries not being associated with notes
- Fix 'paper' button creating 2 pages

#### Embedded Components

- Flip-card for studying (useful with sharing)
- AI triggers for generating study guides and flash cards
- AI trigger modes: auto, confirm, never

#### Documentation

- Card component documentation

#### Desktop

- AI summarization
- Scroll restoration in landscape mode
- Sync method for creating notes from file system

#### iPad

- Ensure note loads in portrait view

#### Webviews

- Ensure all taggables have 'cursor-pointer' class

#### Website

- Update article to match app content
- Fix admonition padding issue

#### Conundrum

- Download **Crafting Interpreters** for offline reading

#### Performance

- Move regex queries to `HashMap<String, bool>`

#### Release

- Ensure all tests pass (Swift, Typescript, Rust)
- Review Apple's nomination process for featured status

#### AI Summary

- Handle light mode for `AISummaryContainer`
- Force regeneration of summaries
- React property parsers for better AI input

#### Offline

- Fix italic field in title syntax
- Relay detailed errors to webview
- Create `subtle` property for `Tabs` component
- Handle `Icon` component with dynamic loading
- Update iPad app with `ErrorStateReset` event
- Add `output_format` property for conditional rendering
- Create `SiaString` class for string compression
- Fix new note being set too late
- Handle initial note refresh issues
- Apply note's front matter summary to new summary class
- Handle new line items not working on initial load
- Global search page on Mac
- Fix note being marked as modified on view
- Set AI availability states
- Generate comment syntax parser
- Generate R3 vector and summary for notes

### Paper Additions

- Derivation of $\frac{dx}{x} = \frac{dr}{R}$
- Spatal tension giving rise to $\alpha$: $2 \frac{R_{\oplus}}{\left( 2 G M_{\oplus} \right)^{1/3}} = \frac{1}{\alpha}$

---

This summary prioritizes the most critical tasks to ensure the release within 2-4 weeks. Focus on the high-priority tasks first, then move to the pre-release milestones and other components. Break down larger tasks into smaller, manageable chunks to stay on track with your ADHD brain. Good luck!