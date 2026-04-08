# Daily Focus for My ADHD-Ass

## Priority Tasks

### 1. Equation Tag Parsing and Components
- Implement equation tag components
- Enable click-to-scroll functionality for equations if the id is present
- Handle parsing of equation tag ids using new syntax to keep the id bound to the equation in the AST

### 2. AI Triggers and Summarization
- Create 'triggers' similar to the `Docs??` concept for AI-generated content
- Implement `FlusterAI.generateStudyGuide()` and `FlusterAI.createFlashCards()`
- Handle light mode of `AISummaryContainer` card
- Handle force regeneration
- Handle React property parsers for AI integration

### 3. Offline and Performance
- Handle inline-code parser and slugger implementation for all headings
- Move regex queries in `get_component_map` to a `HashMap<String, bool>`
- Fix issue with Safari keeping around 20 instances of the editor view
- Handle command palette not changing state fields when the editor is focused

### 4. Webviews and Syncing
- Make sure all taggables have a 'cursor-pointer' class
- Work on syncing method to create notes from file system
- Handle math output issue with the snapshot

### 5. Documentation and Release
- Update docs to reflect upcoming Apple release
- Review YouTube link about creating 'Nominations' for Apple's editor team
- Handle AI summary output format and color scheme note changes

## High Priority
- Fix issue with iPad app loading in portrait view
- Handle bug with color scheme note changing editor theme
- Add `output_format` property to `ConundrumInput.state`
- Create `SiaString` class in Typescript for compression and comparison

## Medium Priority
- Work on AI summarization and syncing method
- Handle issue with new note being set late
- Fix issue with Mac `NoteDetailSheet` requiring refresh
- Write tests for all `-ignoreParser` flags

## Low Priority
- Handle issue with new line items not working on initial new line
- Create quick plot generator for mean conundrum parsing time
- Handle bug that requires initial note to be refreshed
- Handle issue with dictionary entries not being found

## Notes
- Make sure to parse title for markdown model with plainText output
- Store an alternative title in markdown format next to the model if needed
- Apply the note's front matter summary to the new summary class if one exists

## Deadline
- Release the application within the next 2 weeks
- All tests must be passing for Swift, Typescript, and Rust
- Review and update docs to reflect upcoming Apple release
- Handle AI availability states and generate note summary

## Tools
- Use `FlusterAI.generateStudyGuide()` and `FlusterAI.createFlashCards()` for AI-generated content
- Use `SiaString` class in Typescript for compression and comparison
- Use `AISummaryContainer` card for AI summary output
- Use `output_format` property to conditionally render each item as either plain text, inline markdown, markdown or mdx

## Final Thoughts
- Prioritize tasks that have the most impact on the release date
- Focus on high priority tasks first
- Make sure to handle all bugs and issues before release
- Use the tools provided to help with AI integration and documentation
- Stay focused and organized to meet the release deadline

