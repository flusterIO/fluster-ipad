# Daily Focus for My ADHD-Ass

## Priority Tasks to Release Within 2 Weeks

### 1. Equation Tag Parsing and Components
- Complete parsing of equation tag ids using new syntax to keep the id bound to the equation in the AST.
- Implement equation tag components.
- Enable click-to-scroll functionality for equations if the id is present.
- Fix bugs with bold, italic, and bold-italic text parsing.
- Handle math block escapability so `$` can be used in user content reliably.

### 2. Emoji and Component Integration
- Move `ComponentMap` to `DashMap<String, ...>` to allow including other components.
- Add full `sizable` props back to emoji component for positioning.
- Enable nested documentation in emoji docs.
- Handle nested equations being numbered improperly by moving `ParseState` to a `Mutex`.

### 3. Documentation and Syntax Fixes
- Fix syntax docs that are completely broken, especially with `-- title="my_title"` syntax.
- Double check Tabs docs as nested components aren't rendering.
- Move away from pre-parsing docs as a whole to support multiple outputs.
- Fix bug with nested equations being numbered improperly.

### 4. Component Migration and Features
- Move over remaining components: Container, HrWithChildren, Grid, and AINoteSummary.
- Add Table of Contents (TOC) feature.
- Implement Table (GFM if performance allows), Strikethrough text, Escapable math blocks, and Escapable strings.
- Reimplement Paragraphs.

### 5. Bug Fixes and Performance
- Fix bibliography entries not being associated with notes properly.
- Create 'paper' button on desktop that doesn't create 2 pages.
- Handle scroll restoration on desktop app in landscape mode.
- Sync method to create notes from file system.
- Fix iPad issue with note loading in portrait view.
- Handle Safari keeping around 20 editor view instances.
- Fix command palette not updating state fields when editor is focused.
- Handle math output issue with snapshot.
- Fix issue with title syntax implementing italic field.

### 6. AI and Documentation Enhancements
- Create AI triggers for generating study guides and flash cards.
- Implement light mode for `AISummaryContainer` card.
- Handle force regeneration of AI summaries.
- Handle React property parsers for reliable component parsing.
- Add `subtle` or `underline` property for `Tabs` component.
- Create searchable `Icon` component for dynamic icon loading.
- Verify iPad app functionality with error state reset and initial state function.
- Add `output_format` property to `ConundrumInput.state` for conditional rendering.
- Create `SiaString` class in Typescript for string compression and comparison.
- Start setting AI availability states and adjust `AiContainerPhase1...` component.

### 7. Offline and Search Features
- Handle slugger implementation for all headings to generate ids during parsing.
- Move rest of components over to Rust.
- Fix issue with new note being set too late.
- Handle bug requiring initial note refresh.
- Handle global search page on Mac with toggle for search type.
- Fix issue with note being set as modified just by viewing it in Mac app.
- Write tests for all `-ignoreParser` flags.

## Release Preparation
- Ensure all tests are passing for Swift, Typescript, and Rust.
- Review YouTube video on creating 'Nominations' for Apple's editor team.
- Update docs to reflect upcoming Apple release.
- Fix admonition padding issue on website and in main app.
- Download **Crafting Interpreters** for offline reading.
- Move regex queries in `get_component_map` to `HashMap<String, bool>` in Rust.
- Handle offline editor instances and inline-code parser.
- Create unique error enum for Conundrum.
- Handle pre-parsing of documentation and initial docs with new internal CLI method.
- Fix issue with Safari keeping around 20 editor view instances.
- Handle command palette not updating state fields when editor is focused.
- Handle math output issue with snapshot.
- Fix issue with title syntax implementing italic field.
- Relay detailed errors to users in webviews.
- Handle `Icon` component that dynamically loads icons from a library.
- Verify iPad app functionality with error state reset and initial state function.
- Add `output_format` property to `ConundrumInput.state` for conditional rendering.
- Create `SiaString` class in Typescript for string compression and comparison.
- Start setting AI availability states and adjust `AiContainerPhase1...` component.
- Handle global search page on Mac with toggle for search type.
- Fix issue with note being set as modified just by viewing it in Mac app.
- Write tests for all `-ignoreParser` flags.

## Final Notes
- Prioritize tasks that will have the most impact on user experience and app functionality.
- Keep daily focus on high-priority tasks to ensure release within the next 2 weeks.
- Regularly review and update the to-do list to reflect progress and any new tasks that arise.
- Stay organized and focused to manage the workload effectively.
- Celebrate small wins to maintain motivation and momentum throughout the development process.

**Remember:** This is a daily focus list, so adjust as needed based on your progress and any new tasks that come up. Stay on track and keep pushing forward to achieve the release goal within the next two weeks!