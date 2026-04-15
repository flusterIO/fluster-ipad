## Daily Focus for ADHD

### Summary of To-Do List

**Priority 1: Core Features and Stability**
- Fix new note first display issue (still unresolved)
- Handle bug requiring details page to be refreshed on Mac
- Setup seeding of notes on desktop app (first app to release)
- Work on integrating typst support (major value add)
- Move remaining components (AINoteSummary, Table of Contents)
- Handle inline-code parser and slugger implementation for all headings

**Priority 2: Parsers and Performance**
- Implement missing parsers: Footnote, Table, Strikethrough, Escapable math blocks, Escapable strings, Paragraphs, Timestamp links
- Move regex queries to HashMap for performance optimization
- Handle math output issue with snapshot
- Fix issue with Safari keeping around 20 editor instances

**Priority 3: AI Features**
- Create AI triggers (auto, confirm, never)
- Implement FlusterAI.generateStudyGuide() and FlusterAI.createFlashCards()
- Handle light mode for AISummaryContainer
- Fix force regeneration issue
- Work on AI summarization and output components

**Priority 4: Desktop and iPad**
- Handle scroll restoration in landscape mode
- Make sure note can be loaded in portrait view on iPad
- Work on syncing method to create notes from file system
- Fix command palette issue with state changes

**Priority 5: Documentation and Release**
- Update docs to reflect Apple release
- Review YouTube video on creating nominations for Apple's editor team
- Handle pre-release checklist (all tests passing)
- Write tests for all -ignoreParser flags

**Other Important Tasks**
- Handle issue with new note being set too late
- Fix bug with color scheme note changing editor theme
- Create SiaString class in Typescript for compression and comparison
- Work on global search page on Mac
- Start setting AI availability states

**Deadline:** Release within next 2 weeks

**Notes:**
- Focus on high-impact tasks first
- Break down complex tasks into smaller steps
- Use the pre-parser for documentation and initial docs
- Prioritize tasks that will have the most impact on user experience
- Keep the release checklist up to date with progress

**Next Steps:**
1. Tackle the most critical bugs first
2. Focus on core features and stability
3. Work on AI features that will add value to users
4. Ensure all tests are passing before release
5. Update documentation and release checklist regularly

**Remember:**
- Stay focused on the most important tasks
- Break down complex tasks into manageable steps
- Keep the release deadline in mind
- Celebrate small wins to stay motivated
- Stay organized and prioritize effectively

**Final Thoughts:**
- This is a crucial time for the project
- Stay focused and driven
- Keep the user experience at the forefront
- Make sure the app is stable and reliable before release
- Stay organized and prioritize effectively

**Let's make this release a success!**