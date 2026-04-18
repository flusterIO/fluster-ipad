# Daily Focus for My ADHD-Ass

## Prioritized Summary of To-Do List

### High Priority (Next 2 Weeks)

1. **Integrate Typst Support** - A major value add that can be handled offline and is on track for release. This will significantly enhance the app's functionality for STEM users.

2. **Setup Seeding of Notes on Desktop App** - This will be the first app released, so ensuring notes can be seeded properly is critical.

3. **Fix Code Block Title-Underscore Parsing Issue** - This is a critical bug that affects the display of code blocks and needs immediate attention.

4. **Handle Auto-Inserted Code Block** - Ensure the code block looks good and is properly formatted.

5. **Work on Remaining Parsers** - Specifically, focus on CodeBlock, Footnote, Table, Strikethrough text, Escapable math blocks, Escap, Paragraphs, Line Item with various checked states, and Timestamp link syntax.

6. **AI Summarization and Triggers** - Implement AI triggers for generating study guides and flash cards. Ensure these can be set to auto, confirm, or never modes.

7. **Handle Light Mode for AISummaryContainer** - While dark mode looks good, light mode needs adjustments to ensure usability.

8. **Fix Bug with Note Display on Mac** - The new note first display issue is still unresolved and needs to be fixed.

9. **Ensure Summary Can Be Accepted or Declined from UI** - This is crucial for user interaction and feedback.

10. **Handle Scroll Restoration on Desktop App in Landscape Mode** - This will improve the user experience when switching between orientations.

### Medium Priority

- **Move Remaining Components** - Ensure all components are moved over and integrated properly.

- **Fix Bug with Bibliography Entries** - Ensure bibliography entries are correctly associated with notes.

- **Handle Webview Components** - Ensure all taggables have a 'cursor-pointer' class for clickability.

- **Update Documentation** - Reflect upcoming Apple releases and fix any padding issues in the main app.

- **Work on Offline Functionality** - Ensure the app works seamlessly offline, especially for reading and note-taking.

### Low Priority

- **Performance Optimization** - Move regex queries to a HashMap for faster lookups.

- **UI/UX Enhancements** - Improve the overall user experience with better design and functionality.

- **Testing and Debugging** - Ensure all tests are passing across Swift, Typescript, and Rust.

## Release Checklist

- **All Tests Passing** - Ensure Swift, Typescript, and Rust tests are all passing before release.

- **Review Apple's Nominations Guide** - Prepare the back story and features for potential Apple feature recognition.

- **Handle AI Summary Regeneration** - Ensure force regeneration works properly.

- **Handle React Property Parsers** - Improve component parsing reliability for better AI integration.

- **Verify iPad App Functionality** - Ensure the iPad app is working correctly, especially with error state handling.

## Additional Notes

- **Offline Reading of Crafting Interpreters** - Download as much of the book as possible for offline reading, especially as the memory layer implementation approaches.

- **Error Handling in Webviews** - Relay detailed errors to users in a type-safe manner.

- **Icon Component Development** - Create a searchable, lazy-loaded list of available icons.

- **Note Detail Sheet Refresh Issues** - Ensure the NoteDetailSheet works correctly without requiring refreshes.

- **Global Search Page on Mac** - Implement a simple search field with toggle for search type.

- **AI Availability States** - Adjust components to reflect AI availability and generate note summaries.

- **Comment Syntax Parser** - Generate a parser to handle commented-out notes for users.

- **R3 Vector and Summary Storage** - Store R3 vectors and summaries with a 'valid' property to track changes.

- **SiaString Class in Typescript** - Implement a class for string compression and comparison to improve benchmarking.

- **Output Components for Markdown Elements** - Ensure all parsed markdown elements have proper output components.

- **Tests for -ignoreParser Flags** - Write comprehensive tests for all ignore parser flags.

This prioritized list ensures that the most critical tasks are addressed first, allowing for a smooth release within the next two weeks. Focus on high-priority items to meet the release deadline and ensure a robust, user-friendly application.