# To-Do

## Desktop

- [ ] Move all parsing to preParse method.
  - [ ] Remove parsing on body change and just handle it on the editor change event. THis will remove the requirement for the NoteModel in the Contentview.
- [ ] Remove `@AppStorage` from main struct to avoid modelContext issue again.
- [ ] Add snippet support for front matter, especially for `topic` and `subject` autocomplete.
- [ ] Fix issue with model container now that it's being passed to the secondary window.
- [ ] Resume by moving all interactions with the modelContext from the command palette window to a function that executes in the primary window's context instead of trying to pass the context down piece by piece. Moving all dependence on the modelContext might resolve the current issue.
- [ ] Add 'escape-to-close' functionality back to command palette.

- [x] Handle updating of editor state:
  - [x] Dark Mode
  - [x] Editor Keymap
  - [x] Editor Theme (Dark)
  - [x] Editor Theme (Light)

- [ ] Fix issue with dictionary entry parsing. This might be applicable to all 'pre-parsing' functionality.

- [ ] Implement error handling method similar to red-thread for all embeddable components that aren't auto inserted.

- [ ] Handle scroll restoration on desktop app while in landscape mode.

- [ ] Work on syncing method to create notes from file system.

## iPad

- [ ] Add script to set `WebviewEnvironment.IPad` so that the loading class will still be appended.

## Documentation

- [ ] Add section to `quick_reference.mdx` about front matter.
