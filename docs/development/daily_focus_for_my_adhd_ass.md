## Daily Focus for My ADHD-Ass

### Summary of To-Do List

#### **Priority Tasks (Next 2 Weeks)**

1. **Equation Tag Parsing & Components**
   - Implement equation tag components.
   - Handle parsing of equation tag ids using new syntax.
   - Enable click-to-scroll functionality for equations if the id is present.

2. **Bug Fixes & Documentation**
   - Fix syntax docs (especially with `-- title="my_title"` syntax).
   - Double-check Tabs docs and move away from pre-parsing the docs as a whole.
   - Fix admonition padding issue on website and main app.

3. **Component Migration & New Features**
   - Move remaining components (Grid, AINoteSummary, Table of Contents) to Rust.
   - Implement `FlusterAI.generateStudyGuide()` and `FlusterAI.createFlashCards()`.
   - Create 'triggers' for AI-generated content with modes: `auto`, `confirm`, `never`.

4. **Performance & Offline Support**
   - Move regex queries to `HashMap<String, bool>` for faster lookup.
   - Handle slugger implementation for all headings to generate unique ids.
   - Fix issue with Safari keeping around 20 instances of the editor view.

5. **UI/UX Improvements**
   - Ensure all taggables have `cursor-pointer` class.
   - Handle light mode for `AISummaryContainer` card.
   - Fix issue with command palette not updating state fields when editor is focused.

6. **Testing & Release Preparation**
   - Ensure all tests (Swift, Typescript, Rust) are passing.
   - Review YouTube video on creating 'Nominations' for Apple's editor team.
   - Prepare for release by reviewing pre-release checklist.

#### **Medium Priority Tasks**

- Work on AI summarization and syncing method to create notes from file system.
- Handle issue with iPad app loading in portrait view.
- Fix issue with `Icon` component dynamically loading icons from a library.
- Create `SiaString` class in Typescript for string compression and comparison.
- Handle math output issue with the given snapshot.
- Fix issue with the `-- title="my_title"` syntax causing broken docs.

#### **Low Priority Tasks**

- Work on webview note details to ensure all taggables are clickable.
- Update website and docs to reflect upcoming Apple release.
- Handle bug with color scheme note changing editor theme.
- Write tests for all `-ignoreParser` flags.

### Notes

- Focus on high-priority tasks first to ensure we meet the 2-week release deadline.
- Break down complex tasks into smaller, manageable chunks.
- Use the `output_format` property to conditionally render note content as plain text, inline markdown, markdown, or mdx.
- Ensure all AI-generated content is properly formatted and integrated into the app.
- Test all components thoroughly before release to avoid any last-minute issues.

### Final Thoughts

This is a critical release, so it's important to stay focused and organized. Prioritize tasks that have the most impact on the user experience and ensure that all components are working as expected. Good luck with the release!

