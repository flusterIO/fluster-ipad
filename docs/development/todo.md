# To-Do

## Bug Log

**_ PRIORITY PRIORITY PRIORITY _**

- [x] Make absolute sure the editor does not completely fail when the note fails to parse when an invalid component is attempted to render. There absolutely _needs_ to be a backup... the note just can't be lost forever.
- [ ] Fix issue with bibliography entries not being associated with note properly.
- [ ] Create 'paper' button on desktop is creating 2 pages.

### Embedded Components

- [ ] Responsive Grid
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

- [ ] Card component documentation

## Desktop

- [ ] Work on AI summarization.
- [ ] Review [this](https://www.ditto.com/blog/running-a-react-web-app-in-an-ios-app) and see how difficult it would be to get rid of this single page build issue. That would expand the capabilities almost infinitely...
- [ ] Handle rest of zod based props
  - [x] Make sure `Emphasis` based colors are working being set from talwind color variable.
  - [x] Make sure sizable props are working, particularly with admonition.
- [ ] Pick back up by handling this horrible user experience when a bib entry is associated with a note.
  - [ ] Move list to lazy list. This is almost surely rendering every item in the list causing the big freeze.
- [ ] Add snippet support for front matter, especially for `topic` and `subject` autocomplete.
- [ ] Resume work on new bibliography rust package. Create the necessary functions to transform a bibtex entry to it's formatted representation.
  - [ ] Use that formatted citation to create a new field.
  - [ ] Create necessary functions to read value from data, use this to populate a 'notes' or description section in the search result card.
  - [ ] Then work on iPad paperkit implementation. There seems to be more documentation for that, so try to get that working first before handling state update on mac.
- [x] Fix issue with model container now that it's being passed to the secondary window.
- [x] Add 'escape-to-close' functionality back to command palette.
- [x] Fix issue with dictionary entry parsing. This might be applicable to all 'pre-parsing' functionality.
- [ ] Implement error handling method similar to red-thread for all embeddable components that aren't auto inserted.
- [ ] Handle scroll restoration on desktop app while in landscape mode.
- [ ] Work on syncing method to create notes from file system.

## iPad

**_RESUME_**

- [ ] Fix issue with bibliography entries not being associated with note properly.
- [ ] Make sure note can be loaded when ipad is initially in portrait view. It's currently stuck in loading mode.

## Webviews

### Note Details

- [ ] Make sure all taggables have a 'cursor-pointer' class to indicate they are clickable.

## Documentation

- [ ] Add section to `quick_reference.mdx` about front matter.
- [ ] Create 'zod' to markdown table or 'componentconfig' to markdown table component and related function. Create a component that is passed in only on docs pages that makes takes that zod schema and generates a react table.
- [ ] Use `Component?` and `Component??` similar to iPython with a new PreParser and automatically insert component documentation here.

## Website

- [x] Update article to same article as what's included in the app if all components are imported.
- [ ] Update docs to reflect upcoming Apple release even more.
- [ ] Fix admonition padding issue on website and now probably in the main app.

## Language

- [ ] Create package after researching architecture:
  - [x] [Chumsky](https://docs.rs/chumsky/latest/chumsky/guide/) for creating a parser.
  - [x] [Miette](https://crates.io/crates/miette) for error reporting, and possibly... at least acording to Gemini, intellisense in codemirror? It said to use the error state that includes suggestions, but that sounds sketchy before I see it.

---

## Performance

- [ ] Move all of regex queries in the `get_component_map` file to a `HashMap<String, bool>` that is generate in Rust now that the component names are being set as an enum.
  - [ ] Just create another enum that handles the internal & documentation components and use those instead of strings, and then write the Rust code to query for each and return a hashmap of `<ComponentName, bool>` along with the parsing results so that whole pre-render query can be moved to a lookup table.

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

# Today

- [ ] Handle rest of iPad webviews
  - [ ] Make sure all state is working and responding to change.
  - [ ] Add in desperately needed save method to iPad settings.
- [ ] Move rest of webviews over to using redux (note-details, dictionary).
- [ ] Move iPad over to using schema, shared webview storage and redux state.
  - [ ] Reimplement all iPad webviews from scratch, **again** to reflect the way a single container is used in the MacOS application. Now that state is shared globally that's the best way to reliably handle state without repeating the same shit over and over.
- [ ] Move on to Founation Models and AI-Parser.
