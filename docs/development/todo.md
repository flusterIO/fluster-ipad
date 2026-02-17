# To-Do

## Desktop

- [ ] Add snippet support for front matter, especially for `topic` and `subject` autocomplete.
- [x] Fix issue with model container now that it's being passed to the secondary window.
- [ ] Add 'escape-to-close' functionality back to command palette.

- [x] Fix issue with dictionary entry parsing. This might be applicable to all 'pre-parsing' functionality.

- [ ] Implement error handling method similar to red-thread for all embeddable components that aren't auto inserted.

- [ ] Handle scroll restoration on desktop app while in landscape mode.

- [ ] Work on syncing method to create notes from file system.

## iPad

- [ ] **_RESUME_** tomorrow by getting iPad webviews back in order.
  - [ ] Move the iPad implementation to the much cleaner setup used in the desktop app with the functions being passed in as clean onLoad and messageHandler functions in the parent webview struct.
- [ ] Add script to set `WebviewEnvironment.IPad` so that the loading class will still be appended.
- [ ] Make sure note can be loaded when ipad is initially in portrait view. It's currently stuck in loading mode.

## Documentation

- [ ] Add section to `quick_reference.mdx` about front matter.
- [ ] Create 'zod' to markdown table or 'componentconfig' to markdown table component and related function. Create a component that is passed in only on docs pages that makes takes that zod schema and generates a react table.

## Website

- [x] Update article to same article as what's included in the app if all components are imported.
- [ ] Fix admonition padding issue on website and now probably in the main app.

## Language

- [ ] Create package after researching architecture:
  - [x] [Chumsky](https://docs.rs/chumsky/latest/chumsky/guide/) for creating a parser.
  - [x] [Miette](https://crates.io/crates/miette) for error reporting, and possibly... at least acording to Gemini, intellisense in codemirror? It said to use the error state that includes suggestions, but that sounds sketchy before I see it.

---

# Today

- [x] Verify that rest of article is good
  - [x] Post article to website and verify it's good in prod.
- [x] Make sure all iPad webviews functioning as expected, especially bibliography editor.
- [x] Resume finish of searchability section in docs.
- [ ] Resume MacOS app
- [ ] Check response on StackOverflow about whitespace.
