# To-Do

## Desktop

- [x] Move all parsing to preParse method.
  - [x] Remove parsing on body change and just handle it on the editor change event. THis will remove the requirement for the NoteModel in the Contentview.
- [x] Remove `@AppStorage` from main struct to avoid modelContext issue again.
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

---

# Today

- [ ] Handle update of frontmatter parsing in rust.
  - [ ] Generate tables from frontmatter directly for docs, dont' try to keep it up to date.
- [ ] Create front-matter snippets mentioned in docs.
- [ ] Resume finish of searchability section in docs.
