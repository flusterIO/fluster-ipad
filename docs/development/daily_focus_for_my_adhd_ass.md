# Daily Focus for My ADHD-Ass

## Summary of To-Do List

### Prioritized Tasks

1. **Handle Line Item Parsing Steps**
   - Read entire line to end of line or EOF using preceeded combinator to match and consume for required indentation.
   - Return an enum `Empty, Line(IndentedLine)` for empty lines.
   - Create a reliable, well-tested function to rejoin indented lines after parsing.

2. **Fix Remaining Bugs**
   - Ensure new note first display issue is resolved.
   - Handle Grid issue with integer parser to prevent floats from breaking the component.
   - Fix Blockquote bug with multi-line entries.
   - Handle issue with inline underline component breaking out of paragraphs.

3. **Implement Missing Parsers**
   - Implement parsers for Line Item with checkbox, Plain line item, Numbered line item, and Full table.
   - Handle missing parsers for CodeBlock, Footnote, Table, Strikethrough text, Escapable math blocks, Escapable strings, Paragraphs, and Timestamp links.

4. **Component Moves and Additions**
   - Move over remaining components like AINoteSummary and Table of Contents (TOC).
   - Work on integrating Typst support for offline use.

5. **AI Features**
   - Create AI triggers for generating study guides and flashcards.
   - Implement AI summary features with light mode and force regeneration.

6. **Offline and Performance Improvements**
   - Fix syntax for italic fields in titles.
   - Move regex queries to a HashMap for faster lookups.

7. **Release Preparation**
   - Ensure all tests are passing for Swift, Typescript, and Rust.
   - Review Apple's nomination guidelines for potential feature recognition.

### Key Focus Areas

- **Bug Fixes**: Prioritize resolving issues that affect user experience, such as new note display and Grid component problems.
- **Parser Implementation**: Focus on completing missing parsers to ensure robust content handling.
- **AI Integration**: Work on AI triggers and summaries to enhance the app's functionality.
- **Performance Optimization**: Move regex queries to a HashMap to improve performance.

### Timeline

- **Next 2 Weeks**: Focus on completing the most critical tasks to ensure a smooth release. Prioritize bug fixes, parser implementation, and AI features.

This summary is designed to help maintain focus and ensure that the most important tasks are addressed first to meet the release deadline.