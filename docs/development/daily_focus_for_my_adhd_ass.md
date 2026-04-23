## Daily Focus for ADHD

### Prioritized Tasks

1. **Integrate Typst Support**
   - This is a major value add that can be handled offline and is critical for release.

2. **Setup Seeding of Notes on Desktop App**
   - This will be the first app released, so it's essential to get this right.

3. **Move Remaining Components**
   - Focus on moving over the `AINoteSummary` component and any other remaining components listed in the todo.

4. **Work on Remaining Parsers**
   - Prioritize the CodeBlock, Footnote, Table, Strikethrough text, Escapable math blocks, Escapable strings, Line Item, and Timestamp link syntax parsers.

5. **Fix Critical Bugs**
   - Ensure the bibliography entries are properly associated with notes.
   - Resolve the issue with the 'paper' button creating two pages.
   - Fix the bug where new notes require a refresh to load data.

6. **Improve UI/UX**
   - Make sure the summary can be accepted or declined from the UI appropriately.
   - Ensure all taggables have a 'cursor-pointer' class to indicate they are clickable.

7. **Handle AI Summarization**
   - Work on AI summarization features and ensure AI triggers can be set to auto, confirm, or never modes.

8. **Offline Functionality**
   - Fix the issue with italic fields in the title and ensure the webview can relay detailed errors to the user.

9. **Performance Improvements**
   - Move regex queries to a `HashMap<String, bool>` for faster lookups.

10. **Release Preparation**
    - Ensure all tests are passing for Swift, Typescript, and Rust.
    - Review the YouTube link for Apple's editor team nominations and prepare the back story for the app.

### Additional Notes

- Focus on high-impact tasks that will have the most significant effect on the user experience.
- Break down larger tasks into smaller, manageable sub-tasks to maintain focus and momentum.
- Regularly review progress and adjust priorities as needed to stay on track for the release within the next two weeks.

Let's tackle these tasks one by one and keep the momentum going!