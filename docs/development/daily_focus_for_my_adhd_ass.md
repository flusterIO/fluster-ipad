# Daily Focus for ADHD-Ass

## Summary of To-Do List

### High Priority Tasks

1. **Fix new note first display issue** - This issue is still unresolved and needs immediate attention.
2. **Handle rendering code to HTML** - This is critical for the application's functionality.
3. **Move over all KateX font methods to use binary stored data** - This will break the standalone app, so it's a high priority.
4. **Handle Grid by fixing issue with integer parser** - This issue is causing the Grid component to fail when props are valid.
5. **Handle Blockquote bug with multi-line entries** - This is a known issue that needs to be resolved.

### Medium Priority Tasks

1. **Write JavaScript to actually copy code from code block** - This will allow other applications to pick up the event.
2. **Handle issue with inline underline component breaking out of paragraph** - This is a pain point that needs to be resolved.
3. **Reimplement `window.setBibtexEditorContent`** - This is a buffer operation that's currently missing.
4. **Handle rest of summary UI** - This will allow users to accept or decline summaries appropriately.

### Low Priority Tasks

1. **Work on integrating typst support** - This is a major value add that can be handled offline.
2. **Setup seeding of notes on desktop app** - This will be the first app released.
3. **Move over rest of components** - This will allow the application to function properly.
4. **Work on remaining parsers** - This will allow the application to handle more content types.

### Bug Fixes

1. **Fix issue with bibliography entries not being associated with note properly** - This is a known issue that needs to be resolved.
2. **Create 'paper' button on desktop** - This is creating 2 pages, which is a bug.
3. **Fix admonition padding issue** - This is an issue that needs to be resolved.

### Documentation

1. **Card component documentation** - This needs to be updated to reflect the latest changes.

### Desktop App

1. **Work on AI summarization** - This will allow users to get summaries of their notes.
2. **Handle scroll restoration on desktop app while in landscape mode** - This is a known issue that needs to be resolved.
3. **Work on syncing method to create notes from file system** - This will allow users to create notes from the file system.

### iPad App

1. **Make sure note can be loaded when iPad is initially in portrait view** - This is a known issue that needs to be resolved.

### Webviews

1. **Make sure all taggables have a 'cursor-pointer' class** - This will indicate that they are clickable.

### Website

1. **Update article to same article as what's included in the app** - This will ensure consistency.
2. **Fix admonition padding issue** - This is an issue that needs to be resolved.

### Conundrum

1. **Download as much of Crafting Interpreters as you can for offline reading** - This will allow users to read the book offline.

### Performance

1. **Move all of regex queries in the `get_component_map` file to a `HashMap<String, bool>`** - This will improve performance.

### Release

1. **All tests passing** - This is critical for the release.
2. **Review YouTube link and section about creating 'Nominations' for being featured by Apple's editor team** - This is important for marketing.
3. **Handle light mode of `AISummaryContainer` card** - This needs to be fixed.
4. **Handle force regeneration** - This is not working and needs to be fixed.
5. **Handle React property parsers** - This will allow components to be parsed more reliably.
6. **Handle offline issues** - This includes fixing the syntax for italic fields in the title and ensuring errors are relayed to the user in the webview.

## Conclusion

This is a comprehensive list of tasks that need to be completed to ensure the release of the application within the next two weeks. The most important tasks are prioritized, and the rest are organized by category. The release checklist includes all tests passing, a review of the YouTube link, and handling of light mode and force regeneration. The offline issues are also addressed, including the syntax for italic fields in the title and ensuring errors are relayed to the user in the webview. The release is scheduled for the next two weeks, and the application is expected to be fully functional by then.