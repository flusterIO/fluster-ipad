# To-Do

### Embedded Components

- [ ] Responsive Grid
  - [ ] This is currently completely non-functional. Handle the column gathering function tomorrow.
- [ ] `CenterCard` that centers the content within it, or a plain, 'fd-card-background' container with sizableObject props and some decent defaults with a default 'flex-center' behavior.
- [ ] `Hr` that accepts children to inline as small text.
  - [ ] Already started, but need to handle all of the config boilerplate.
- [ ] `Tabs` component that just accepts children and a context provider to set the tab content, not a prop from the user.

#### Documentation

**_CURRENT_**

- [ ] Handle `InContentDocumentationContainer`'s retreival of embedded docs.
- [ ] Use an `Enum` to define component names so that they can be used to create a new parser that can check for `new line<component-name>?` or `\n<component-name>??` to print out the embedded docs like Jupyter. You can insert the component documentation directly from the parser package.
- [ ] Create new 'documentation' parser and add that to the list. Make sure there is an option to disable it in some cases to improve performance on inline mdx.
- [ ] Create `InContentDocumentation` component that creates a card that accepts optional properties, and the children as markdown to create the necessary docs. Consider restyling the prose text size here to make things more 'docs' like... straight and to the point.

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

- [ ] Resume MacOS app
  - [ ] Implement desktop app paperkit.
  - [ ] Move iPad drawing from pencilkit to paperkit for more note-taking style annotations.
  - [ ] Get bibtex snippets working.
- [ ] Check response on StackOverflow about whitespace.
