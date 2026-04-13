## Daily Focus for ADHD

### Summary of To-Do List

#### High Priority Tasks

1. **Equation Tag Parsing and Components**
   - Implement equation tag components
   - Handle parsing of equation tag ids using new syntax
   - Enable click-to-scroll functionality for equations if the id is present
   - Fix bugs with bold, italic, and bold-italic text parsing

2. **Math Block Escapability**
   - Handle math block escapability so `$` can be used in the user's regular note content reliably

3. **Fix Syntax Docs**
   - Fix syntax docs... they're completely fucking broken... (likely arising from the `-- title="my_title"` syntax)
   - Double check Tabs docs too, as the nested components aren't rendering since the docs aren't being rendered as Children

4. **Nested Equations and ParseState**
   - Fix bug with nested equations being numbered improperly
   - Move `ParseState` from a `RefCell` to a `Mutex`

5. **Blog Page and Launch Announcement**
   - Handle blog page on website and first blog post announcing the launch of Fluster!

#### Medium Priority Tasks

1. **Components and Documentation**
   - Move over rest of components below:
     - AINoteSummary
   - Add Table of Contents (TOC)
   - Update docs to reflect upcoming Apple release

2. **Performance Improvements**
   - Move all of regex queries in the `get_component_map` file to a `HashMap<String, bool>`

3. **AI Features**
   - Create 'triggers' similar in concept to the `Docs??` concept that can be replaced with on-board AI generated content
   - Make sure ai-triggers can be set to 1 of 3 modes: `auto`, `confirm`, `never`

4. **Offline and Syncing**
   - Handle slugger implementation for _all_ headings
   - Setup a unique error enum for Conundrum
   - Handle issue with Safari keeping around 20 instances of the editor view

#### Low Priority Tasks

1. **Webview and Website Improvements**
   - Make sure all taggables have a 'cursor-pointer' class to indicate they are clickable
   - Fix admonition padding issue on website and now probably in the main app

2. **Testing and Release**
   - Ensure all tests are passing for Swift, Typescript, and Rust
   - Review [this](https://www.youtube.com/watch?v=fkeUvZ4NRhg) link and the section at ~5:00 about creating 'Nominations' for being featured by Apple's editor team

3. **Other Tasks**
   - Handle light mode of `AISummaryContainer` card or whatever it's called
   - Handle force regeneration, because that's not working...
   - Handle React property parsers so that the components can be more reliably parsed and the input more reliably inserted into AI

### Release Plan

- **Pre-Release Checklist**
  - All tests passing
    - Swift
    - Typescript
    - Rust

- **Post Release**
  - Review [this](https://www.youtube.com/watch?v=fkeUvZ4NRhg) link and the section at ~5:00 about creating 'Nominations' for being featured by Apple's editor team

### Notes

- The release is scheduled for within the next 2 weeks
- The daily focus is designed to help with ADHD by prioritizing tasks and keeping the focus on the most important ones
- The summary is written in plain text and will be delivered automatically
- If the task explicitly calls for messaging a specific external recipient, note who/where it should go instead of sending it yourself

### Conclusion

This daily focus is a summary of the to-do list for the Fluster application. It prioritizes the most important tasks to ensure that the application can be released within the next 2 weeks. The summary is written in plain text and will be delivered automatically. If the task explicitly calls for messaging a specific external recipient, note who/where it should go instead of sending it yourself.