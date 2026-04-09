### Daily Focus for My ADHD Ass

**Current Date:** Thursday, April 9th, 2026 — 8:52 AM (America/Chicago) / 2026-04-09 13:52 UTC

**Summary of To-Do List Prioritized for Next 2 Weeks Release:**

#### 🚀 High Priority Tasks

1. **Equation Tag Parsing and Components**
   - Handle parsing of equation tag ids using new syntax to keep the id bound to the equation in the AST.
   - Implement equation tag components.
   - Enable click-to-scroll functionality for equations if the id is present.

2. **Bug Fixes and Performance Improvements**
   - Handle bugs with bold, italic, and bold-italic text parsing.
   - Fix syntax docs that are completely broken, especially related to the `-- title="my_title"` syntax.
   - Move all regex queries in the `get_component_map` file to a `HashMap<String, bool>` for faster lookups.

3. **AI Integration and Features**
   - Work on integrating the new emoji crate directly into the transpiler.
   - Create 'triggers' for AI-generated content (e.g., `FlusterAI.generateStudyGuide()`, `FlusterAI.createFlashCards()`).
   - Make sure AI triggers can be set to 1 of 3 modes: `auto`, `confirm`, or `never`.

4. **UI and Component Development**
   - Work on the `Tabs` component that accepts children and a context provider.
   - Create a flip-card like component for studying.
   - Handle light mode for `AISummaryContainer` card.

5. **Offline and Syncing Features**
   - Handle slugger implementation for all headings to generate unique ids.
   - Setup a unique error enum for Conundrum.
   - Fix issue with Safari keeping around 20 instances of the editor view.

#### 📌 Medium Priority Tasks

- Move remaining components (e.g., `HrWithChildren`, `AINoteSummary`) to Rust.
- Implement `SiaString` class in Typescript for easy compression and comparison.
- Handle math output issue with the provided snapshot.
- Fix issue with the `NoteDetailSheet` requiring refresh on Mac.
- Work on syncing method to create notes from file system.

#### 📝 Documentation and Testing

- Update docs to reflect upcoming Apple release.
- Fix admonition padding issue on website and in the main app.
- Write tests for all `-ignoreParser` flags.
- Ensure all tests are passing (Swift, Typescript, Rust).

#### 📅 Release Preparation

- Review the YouTube link about creating 'Nominations' for being featured by Apple's editor team.
- Handle force regeneration of AI summaries.
- Ensure all tests are passing before release.

**Next Steps:**
- Focus on high-priority tasks first, especially those related to performance, AI integration, and critical bug fixes.
- Break down complex tasks into smaller, manageable chunks to maintain focus.
- Regularly review progress and adjust priorities as needed.

**Note:** This summary is a prioritized list of tasks to ensure the application is released within the next 2 weeks. Focus on the most critical issues first and gradually move to medium and low priority tasks as time permits.

**Remember:** Stay organized, take breaks, and celebrate small wins to maintain motivation and productivity.

**End of Summary**