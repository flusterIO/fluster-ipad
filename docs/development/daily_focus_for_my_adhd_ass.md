## Daily Focus for My ADHD-Ass

### Summary of To-Do List

#### High Priority Tasks

1. **Complete the dictionary entry rendering to HTML** - This is critical for the dictionary page to function properly.
2. **Fix the independent markdown view page** - The tabs are not rendering properly on load, which affects the user experience.
3. **Implement the missing parsers** - This includes list items, tables, footnotes, and other essential elements for documentation to work correctly.
4. **Reimplement `window.setBibtexEditorContent`** - This function is currently missing and needs to be restored for proper bibliography handling.
5. **Ensure the summary UI works** - The summary needs to be acceptably or declineable from the UI, which is a key part of the user interaction.

#### Pre-Release Milestones

- **Get the dictionary entry page working** - This requires the dictionary HTML output parser and a new query method to return properly formatted HTML.
- **All documentation pages back in working order** - This involves parsing and rendering tables, list items, and footnotes.
- **Move over the remaining components** - This includes `AINoteSummary` and other components that are still pending.

#### Up Next

- **Integrate typst support** - This is a major value add that can be handled offline and is essential for the release.
- **Setup seeding of notes on the desktop app** - This will be the first app released, so it's important to get this working.
- **Work on remaining parsers** - This includes code blocks, footnotes, tables, and other elements that are still missing.

#### Bug Fixes and Improvements

- **Fix the issue with bibliography entries not being associated with notes properly** - This is a critical bug that needs to be resolved.
- **Handle scroll restoration on the desktop app while in landscape mode** - This will improve the user experience.
- **Ensure the iPad app works correctly** - The app needs to handle note loading when in portrait view.
- **Improve the AI summarization** - This will enhance the app's functionality and user experience.

#### AI and Documentation

- **Create AI triggers** - These will allow users to generate study guides and flash cards using the app's AI capabilities.
- **Handle the light mode for `AISummaryContainer`** - The dark mode looks great, but light mode needs some adjustments.
- **Handle force regeneration of AI summaries** - This is important for ensuring the summaries are up-to-date.

#### Offline and Performance

- **Fix the issue with the syntax for italic fields in titles** - This is a critical bug that affects the formatting of titles.
- **Move regex queries to a `HashMap<String, bool>`** - This will improve performance by reducing the overhead of regex operations.

#### Release Preparation

- **Ensure all tests are passing** - This includes tests for Swift, Typescript, and Rust.
- **Review the YouTube link about creating 'Nominations' for Apple's editor team** - This is important for increasing the app's visibility and chances of being featured.

#### Additional Tasks

- **Handle the `Icon` component** - This component needs to dynamically load icons from a library and allow for a searchable list of available icons.
- **Verify the iPad app is working again** - This involves several changes to the app's functionality and error handling.
- **Create a `SiaString` class in Typescript** - This class will help with compression and comparison of strings, improving the app's performance.

### Conclusion

The most important tasks are focused on completing the dictionary rendering, fixing the markdown view page, implementing the missing parsers, and ensuring the summary UI works. These tasks are critical for the app's functionality and user experience. The release is scheduled for within the next two weeks, and all pre-release milestones need to be completed to ensure a successful launch.