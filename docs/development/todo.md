# To-Do

## Bug Log

- [x] Fix issue with bibliography entries not being associated with note properly.
- [x] Create 'paper' button on desktop is creating 2 pages.
- [ ]

### Embedded Components

- [x] Responsive Grid
- [ ] `Tabs` component that just accepts children and a context provider to set the tab content, not a prop from the user.
- [ ] Flip-card like component for studying. This will obviously become a lot more useful once sharing is enabled, although notes could be generated with a server scale AI an integrated with the app easily.

#### AI

- [ ] Create 'triggers', similar in concept to the `Docs??` concept that can be replaced with on-board AI generated content. Make it look like a code snippet to trigger the replacement just for future's sake, but this will also make sure that this line doesn't appear alone anywhere just by coincidence.
  - [ ] `FlusterAI.generateStudyGuide()`
  - [ ] `FlusterAI.createFlashCards()`
- [ ] Make sure ai-triggers can be set to 1 of 3 modes:
  - [ ] `auto`
  - [ ] `confirm`
  - [ ] `never`

#### Documentation

**_CURRENT_**

- [x] Card component documentation

## Desktop

- [ ] Work on AI summarization.
- [ ] Handle scroll restoration on desktop app while in landscape mode.
- [ ] Work on syncing method to create notes from file system.

## iPad

**_RESUME_**

- [x] Fix issue with bibliography entries not being associated with note properly.
- [ ] Make sure note can be loaded when ipad is initially in portrait view. It's currently stuck in loading mode.

## Webviews

### Note Details

- [ ] Make sure all taggables have a 'cursor-pointer' class to indicate they are clickable.

## Website

- [x] Update article to same article as what's included in the app if all components are imported.
- [ ] Update docs to reflect upcoming Apple release even more.
- [x] Fix admonition padding issue on website and now probably in the main app.

## Conundrum

- [x] Download as much of **Crafting Interpreters** as you can for offline reading, especially as it gets closer to implementing a memory layer.

---

## Performance

- [ ] Move all of regex queries in the `get_component_map` file to a `HashMap<String, bool>` that is generate in Rust now that the component names are being set as an enum.

---

## Release

### Pre-Release Checklist

- [ ] All tests passing
  - [ ] Swift
  - [ ] Typescript
  - [ ] Rust

#### Post Release

- [ ] Review [this](https://www.youtube.com/watch?v=fkeUvZ4NRhg) link and the section at ~5:00 about creating 'Nominations' for being featured by Apple's editor team. There's a special section for talking about the back story of the app, and this app certainly has a back story...

---

### Ai Summary

- [ ] Handle light mode of `AISummaryContainer` card or whatever it's called. It looks _sick_ in dark mode, but light mode needs some work...
- [ ] Handle force regeneration, because that's not working...
- [ ] Handle React property parsers so that the components can be more reliably parsed and the input more reliably inserted into AI. This will open the door to a **TON** new possibilities.

## Offline

**_PRIORITY:_** Handle this issue with the editor instances first.

- [x] Review Conundrum docs... you were spaced out as shit.
- [x] Handle inline-code parser. Make sure it's last in the list, but it's required to document the note id syntax.
- [x] Setup a unique error enum for Conundrum
- [x] Handle slugger implementation for _all_ headings. Generate an id during the parsing stage if none exists.
- [x] Move rest of components over to Rust.
  - [x] Move away from `ComponentProps` struct as the types will be known and the type conversions would be unnecessary. It's better to keep things typed if they're coming in typed.
- [ ] Handle pre-parsing of documentation and initial docs with new internal cli method. It's already in place, you just need to write the script to call it.
- [ ] Fix issue with Safari keeping around 20 instances of the editor view.
- [ ] Handle issue with command palette not changing a variety of different state fields when the editor is focused, requiring a user navigation away from an then back to the editor page to see the changes.
- [ ] Verify that the iPad app is working again. It will need at least changes to:
  - [ ] Send new `ErrorStateReset` event after manual save request content change on the iPad app so that error state can be cleared by user without requiring navigation away and back to note.
  - [ ] Initial State function
  - [ ] `WebviewContainer.mathjaxUrl` field added and passed around properly.
- [x] Handle move of `to_mdx_component` to `to_fluster_component` in `packages/rust/conundrum/src/lang/runtime/compile_conundrum.rs`.
  - This means that everything will need to implement the `FlusterComponentResult` trait, but this should allow the different output methods then based on the flags passed.
- [ ] Make sure to parse title for markdown model with plainText output to avoid markdown content for text comparison.
  - [ ] Store an alternative title in markdown format next to the model if needed.
- [ ] Handle bug with color scheme note changing editor theme when toggled via the command palette on the editor page anymore for some inexplicable reason...
- [x] Add `output_format` property to `ConundrumInput.state` to conditionally render each item as either plain text, inline markdown, markdown or mdx. Use this to store a stringified, more searchable title alongside the note's content if the stringified title does not match the normal title exactly. This will get around the searchability issue with markdown based titles.
- [x] Create quick plot generator for mean conundrum parsing time.
- [x] Handle issue with `NoteDetailSheet` requiring refresh on Mac to load contents.
- [x] Create `SiaString` class in Typescript for easy compression and comparison of various strings.
  - Use that in the benchmark output instead of storing the whole fucking file... what a brilliant idea that was...
- [x] Fix issue with new note being set _superrrrrr_ late.
- [ ] Handle bug that requires initial note to be refreshed. The first always renders an error.
- [ ] Fix issue with Mac `NoteDetailSheet` requiring a refresh for some reason to load the note's data.
- [x] Handle issue with dictionary entries not being found now for some reason...
- [x] Apply the note's front matter summary to the new summary class if one exists in the user's front matter.
- [ ] Handle issue with new line items not working on initial new line.
- [x] Handle Global search page on Mac. This will be a crucial page moving forward, but for now a simple search field with a toggle that swaps the search type will do.
- [x] Fix issue with note being set as modified just by viewing it in Mac app.
- [ ] Start setting AI availability states.
  - [ ] Adjust `AiContainerPhase1...` component to reflect the user's AI availability.
  - [ ] Move on to generating note summary!
  - [ ] Add comment syntax parser that just strips the content before rendering so you can write commented out notes to the user on the generated code like "t's ok to delete this, your summary was attached to your note".
- [x] Work on output components for all new markdown parsed elements.
- [ ] Write tests for all `-ignoreParser` flags.

## AI

- [ ] Generate comment syntax parser before moving on to sending the AI request since you'll likely need to leave some comments in the user's note.
- [ ] Generate R3 vector in background tasks for each note along with a note summary if that note's summary is empty.
  - [ ] Also store a 'valid' property alongside the summary and R3 vec that can be set to false when the note's content is updated.
