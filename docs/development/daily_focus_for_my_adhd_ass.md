# Daily Focus for My ADHD-Ass

## Summary of To-Do List

### High Priority Tasks

1. **Fix New Note First Display Issue** - This is still unresolved and needs immediate attention.
2. **Handle Rendering Code to HTML** - Critical for the app's functionality.
3. **Move Over All Katex Font Methods to Use Binary Stored Data** - This will break the standalone app, so it's a high priority.
4. **Handle Grid by Fixing Issue with Integer Parser** - The current issue is causing the Grid component to fail when props are valid.
5. **Handle Blockquote Bug with Multi-Line Entries** - This is a known bug that needs to be fixed.

### Medium Priority Tasks

1. **Write JavaScript to Actually Copy Code from Code Block** - This includes emitting an event that can be picked up by other applications, including Fluster.
2. **Handle Issue with Inline Underline Component Breaking Out of Paragraph** - This is a pain point that needs to be resolved.
3. **Reattach Theme Selector to Swift** - Create an independent view that can be added to both Mac and iOS using the Rust generated enums.
4. **Fix Independent Markdown View Page** - This is not rendering at all now, and it's a loading class issue that's never being set to false.

### Low Priority Tasks

1. **Write Script to Gather and Generate Single JavaScript Typescript File from Combined Component Files from Generated Dashmap** - This is a long-term task that can be done once the main issues are resolved.
2. **Handle Missing Parsers** - This includes Line Item with checkbox, Plain line item, Numbered line item, and Full table!
3. **Reimplement `window.setBibtexEditorContent`** - This is a buffer operation that's currently MIA.
4. **Handle Rest of Summary UI** - This includes making sure the summary can be accepted or declined from the UI appropriately.

### Additional Tasks

1. **Work on Integrating Typst Support** - This is a major value add that can be handled offline and in time to release.
2. **Setup Seeding of Notes on Desktop App** - This will be the first app that's released most likely.
3. **Move Over Rest of Components** - This includes Work on remaining parsers.
4. **Handle Performance Issues** - This includes moving all of regex queries in the `get_component_map` file to a `HashMap<String, bool>` that is generated in Rust now that the component names are being set as an enum.

### Release Checklist

1. **All Tests Passing** - This includes Swift, Typescript, and Rust.
2. **Review YouTube Link and Section at ~5:00** - This is about creating 'Nominations' for being featured by Apple's editor team.
3. **Handle Light Mode of `AISummaryContainer` Card** - It looks sick in dark mode, but light mode needs some work.
4. **Handle Force Regeneration** - This is not working and needs to be fixed.
5. **Handle React Property Parsers** - This will open the door to a TON new possibilities.

## Conclusion

The most important tasks are the ones that need immediate attention, such as fixing the new note first display issue and handling rendering code to HTML. These tasks are critical for the app's functionality and need to be prioritized. The medium priority tasks are also important and should be addressed in the coming days. The low priority tasks can be handled once the main issues are resolved. The release checklist includes all the necessary steps to ensure the app is ready for release within the next two weeks.