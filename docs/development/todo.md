# To-Do

## Fixes Before Moving Forward

- [ ] Make sure editor works on all platforms.
- [ ] Make sure `fd-xyz` based classes are working.

## Cross-Platform

- [ ] Move all editor interactions that user defined data to go through flatbuffers for serialization. It seems like it can break in some circumstances, and this will improve reliability _significantly_.

## Desktop

- [ ] Work on AI summarization.
- [ ] Review [this](https://www.ditto.com/blog/running-a-react-web-app-in-an-ios-app) and see how difficult it would be to get rid of this single page build issue. That would expand the capabilities almost infinitely...
- [ ] Handle rest of zod based props
  - [x] Make sure `Emphasis` based colors are working being set from talwind color variable.
  - [x] Make sure sizable props are working, particularly with admonition.
  - [x] Make sure `@expandedMdx/mdx:w-1/3` or similar props are working. They're already in use in the admonition component.
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
- [ ] Fix admonition padding issue on website and now probably in the main app.

## Language

- [ ] Create package after researching architecture:
  - [x] [Chumsky](https://docs.rs/chumsky/latest/chumsky/guide/) for creating a parser.
  - [x] [Miette](https://crates.io/crates/miette) for error reporting, and possibly... at least acording to Gemini, intellisense in codemirror? It said to use the error state that includes suggestions, but that sounds sketchy before I see it.

---

# Today

- [ ] Resume MacOS app
  - [ ] Implement desktop app paperkit.
  - [ ] Move iPad drawing from pencilkit to paperkit for more note-taking style annotations.
  - [ ] Get bibtex snippets working.
- [ ] Check response on StackOverflow about whitespace.
