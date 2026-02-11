# To-Do

## Desktop

- [ ] Fix issue with model container now that it's being passed to the secondary window.
- [ ] Resume by moving all interactions with the modelContext from the command palette window to a function that executes in the primary window's context instead of trying to pass the context down piece by piece. Moving all dependence on the modelContext might resolve the current issue.
- [ ] Add 'escape-to-close' functionality back to command palette.

- [ ] Handle updating of editor state:
  - [ ] Dark Mode
  - [x] Editor Keymap
  - [ ] Editor Theme (Dark)
  - [ ] Editor Theme (Light)

- [ ] Fix issue with dictionary entry parsing. This might be applicable to all 'pre-parsing' functionality.

- [ ] Implement error handling method similar to red-thread for all embeddable components that aren't auto inserted.

- [ ] Handle scroll restoration on desktop app while in landscape mode.

- [ ] Work on syncing method to create notes from file system.
