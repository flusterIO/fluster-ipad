## Daily Focus for ADHD

### Prioritized Tasks

1. **Move over all KateX font methods to use binary stored data** - This will break the standalone app and is critical for release.
2. **Reimplement `window.setBibtexEditorContent`** - It's a buffer operation that's currently MIA.
3. **Handle auto-inserted code block** - It has the potential to look really good, but it definitely needs to be fixed.
4. **Fix code block title-underscore parsing issue**
5. **Work on integrating typst support** - This is a major value add that can be handled offline and is essential for release.
6. **Setup seeding of notes on desktop app** - This will be the first app released.
7. **Handle rendering code to HTML**
8. **Fix new note first display issue** - It's still not resolved!
9. **Handle bug requiring details page to be refreshed on Mac**
10. **Move over the `from_props` methods to accepting a state param** - This will allow breaking up the parsing and compilation phases properly.

### Other Important Tasks

- **Handle rest of summary UI**
- **Make sure summary can be accepted or declined from the UI appropriately**
- **Work on remaining parsers**
- **Handle light mode of `AISummaryContainer` card** - It looks sick in dark mode, but light mode needs some work.
- **Handle force regeneration** - Because that's not working...
- **Handle React property parsers** - So that the components can be more reliably parsed and the input more reliably inserted into AI.
- **Verify that the iPad app is working again** - It will need changes to send new `ErrorStateReset` event after manual save request content change on the iPad app so that error state can be cleared by user without requiring navigation away and back to note.
- **Create `SiaString` class in Typescript** - For easy compression and comparison of various strings.
- **Work on output components for all new markdown parsed elements**
- **Write tests for all `-ignoreParser` flags`**

### Bug Fixes

- **Fix issue with bibliography entries not being associated with note properly**
- **Create 'paper' button on desktop** - It's creating 2 pages.
- **Fix note can be loaded when iPad is initially in portrait view** - It's currently stuck in loading mode.
- **Fix issue with Mac `NoteDetailSheet` requiring a refresh for some reason to load the note's data**
- **Fix issue with new note being set _superrr**
- **Fix issue with dictionary entries not being found now for some reason**
- **Fix issue with note being set as modified just by viewing it in Mac app**
- **Fix issue with new line items not working on initial new line**
- **Fix Global search page on Mac** - This will be a crucial page moving forward.

### Performance

- **Move all of regex queries in the `get_component_map` file to a `HashMap<String, bool>`** - That is generated in Rust now that the component names are being set as an enum.

### Release

- **All tests passing** - Swift, Typescript, Rust
- **Review [this](https://www.youtube.com/watch?v=fkeUvZ4NRhg) link** - And the section at ~5:00 about creating 'Nominations' for being featured by Apple's editor team.
- **Handle light mode of `AISummaryContainer` card** - It looks _sick_ in dark mode, but light mode needs some work...
- **Handle force regeneration** - Because that's not working...
- **Handle React property parsers** - So that the components can be more reliably parsed and the input more reliably inserted into AI. This will open the door to a **TON** new possibilities.

### Offline

- **Fix issue with this syntax** - It's currently trying to implement an italic field in the title.
- **Now that you're getting back great, _detailed_ errors, relay that information to the user in the webview!** - Pipe the error to the webview error display in a type-safe way.
- **Create a new `subtle` or `underline` property for the `Tabs` component** - So that the `Emphasis` variants of `Tabs` component use border under button style instead of that ridiculous full background for bright colors.
- **Handle `Icon` component that dynamically loads icon from that one library using the icon's name** - Create a component that renders a searchable _lazy_ list of all available icons.
- **Verify that the iPad app is working again** - It will need at least changes to send new `ErrorStateReset` event after manual save request content change on the iPad app so that error state can be cleared by user without requiring navigation away and back to note.
- **Handle move of `to_mdx_component` to `to_fluster_component` in `packages/rust/conundrum/src/lang/runtime/compile_conundrum.rs`** - This means that everything will need to implement the `FlusterComponentResult` trait, but this should allow the different output methods then based on the flags passed.
- **Make sure to parse title for markdown model with plainText output to avoid markdown content for text comparison** - Store an alternative title in markdown format next to the model if needed.
- **Handle bug with color scheme note changing editor theme when toggled via the command palette on the editor page anymore for some inexplicable reason...**
- **Add `output_format` property to `ConundrumInput.state`** - To conditionally render each item as either plain text, inline markdown, markdown or mdx. Use this to store a stringified, more searchable title alongside the note's content if the stringified title does not match the normal title exactly. This will get around the searchability issue with markdown based titles.
- **Create quick plot generator for mean conundrum parsing time**
- **Handle issue with `NoteDetailSheet` requiring refresh on Mac to load contents**
- **Create `SiaString` class in Typescript** - For easy compression and comparison of various strings.
- **Fix issue with new note being set _superrr**
- **Handle bug that requires initial note to be refreshed** - The first always renders an error.
- **Fix issue with Mac `NoteDetailSheet` requiring a refresh for some reason to load the note's data**
- **Handle Global search page on Mac** - This will be a crucial page moving forward, but for now a simple search field with a toggle that swaps the search type will do.
- **Fix issue with note being set as modified just by viewing it in Mac app**
- **Start setting AI availability states** - Adjust `AiContainerPhase1...` component to reflect the user's AI availability. Move on to generating note summary! Add comment syntax parser that just strips the content before rendering so you can write commented out notes to the user on the generated code like "t's ok to delete this, your summary was attached to your note".
- **Work on output components for all new markdown parsed elements**
- **Write tests for all `-ignoreParser` flags`**

### AI

- **Generate comment syntax parser before moving on to sending the AI request** - Since you'll likely need to leave some comments in the user's note.
- **Generate R3 vector in background tasks for each note along with a note summary if that note's summary is empty** - Also store a 'valid' property alongside the summary and R3 vec that can be set to false when the note's content is updated.

## Summary

The most important tasks to prioritize for release within the next 2 weeks are:

1. Move over all KateX font methods to use binary stored data.
2. Reimplement `window.setBibtexEditorContent`.
3. Handle auto-inserted code block.
4. Fix code block title-underscore parsing issue.
5. Work on integrating typst support.
6. Setup seeding of notes on desktop app.
7. Handle rendering code to HTML.
8. Fix new note first display issue.
9. Handle bug requiring details page to be refreshed on Mac.
10. Move over the `from_props` methods to accepting a state param.

These tasks are critical for release and should be prioritized first.

Other important tasks include:

- Handle rest of summary UI.
- Make sure summary can be accepted or declined from the UI appropriately.
- Work on remaining parsers.
- Handle light mode of `AISummaryContainer` card.
- Handle force regeneration.
- Handle React property parsers.
- Verify that the iPad app is working again.
- Create `SiaString` class in Typescript.
- Work on output components for all new markdown parsed elements.
- Write tests for all `-ignoreParser` flags.

Bug fixes are also important and should be addressed as soon as possible.

Performance improvements are also needed, especially moving all of regex queries in the `get_component_map` file to a `HashMap<String, bool>` that is generated in Rust.

Release preparations include ensuring all tests are passing and reviewing the link about creating 'Nomin's for being featured by Apple's editor team.

AI-related tasks include generating a comment syntax parser, generating R3 vectors in background tasks, and setting AI availability states.

Overall, the most important tasks to prioritize for release within the next 2 weeks are the ones that are critical for functionality and user experience. These tasks should be addressed first to ensure a successful release.