# To-Do

## In Progress

- [ ] Create separate Emphasis **Component** in Rust.
- [ ] Use docgen to output that component for each Emphasis variant.
- [ ] Make this component applicable to any color group.

---

### Pre-release bugs

- [x] Get conundrum compiling again.
- [x] Re-fucking-implement strikethrough text parser.
- [x] Fix link parser. It's woring, but inserting the wrong text maing the link invalid.
- [ ] Create color component and fix Emphasis docs that are no longer working.
  - **On Layaway until AI creates JSON file**
- [ ] Fix issue with dictionary entry conundrum content parsing.
  - [ ] Finish proper rendering of dictionary page.
- [ ] Get bibliography editor working again.
  - **On layaway until lezer package can be cloned**
- [ ] Write new bibliography snippets and include them for the obvious things for general users like a web page.
  - [ ] Make sure shit bib snippets are not shown.
- [x] Reset dictionary page... you're an idiot, of course it still needs to be wrapped in React... it needs to respond to UI state like darkmode.
- [x] Fix issue with desktop app not updating conundrum content in the editor successfully when the state is changed.
- [ ] Get `wasm` loading in Next.js. That'll be perfect for enabling a web based editor, but it's also crucial for a decent development workflow now that we're rendering straight to html and a React based test page doesn't make as much sense.
- [x] Make blog mobile responsive. That should be the first priority once conundrum content is compiling again.
  - [x] Add drawer in place of sidebar on mobile.
    - Installed `react-device-detect` for the switch
- [x] Get checkbox parser working
- [ ] Get emoji search working again, then check and verify that all documentation compiles.
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)
  - [ ] Add the tabs component into the application somewhere to show it off.
- [ ] Work on initial launch note conversion to `.cdrm`
- [ ] Create various state comparison methods to compare the json data to url query strings, so that it can be done server side. Keep all of the blog state that isn't component local in the url.
- [ ] Work on Emoji search!
- [ ] Once Emoji search and all documentation is in working order, work on the release features:
  - [ ] Initial note seeding
  - [ ] Onboarding toast things with ToastKit or whatever it's called.
  - [ ] Onboarding pages for mac (if time allows and a design works well)
- [ ] Add 'env-file-provider' to configuration if the user provides a 'env_file' key in their config.
- [ ] Add 'col-span' property from 1-12 to the `Sizable` struct.

---

## Pre-Release Milestones

- [ ] Get dictionary entry page working (requires dictionary html output parser and new query method to return properly formatted html)
  - [x] Render dictionary entries to html
  - [ ] Handle rendering of dictionary page in React still, directly from the Swift DB.
- [ ] All documentation pages back in working order (Handle tables, list items and rest of missing parsers)
  - [ ] Parse & Render Table
  - [ ] Parse & Render Lists
    - [x] Unordered (half complete)
    - [ ] Ordered
    - [ ] Checkbox
  - [x] Parse & Render Footnotes (half complete)
  - [ ] Emoji search and other doc specific components working as expected.

### Components that need to move over still

- [x] Container
- [x] HrWithChildren (in jsx form)
- [x] Grid (Have not yet moved over all properties)
- [ ] AINoteSummary

#### Then Add

- [x] Table of Contents (TOC)

---

### Up Next

- [ ] Work on integrating typst support! That's a **major** value add that can actually be handled offline, and in time to release!
- [ ] Setup seeding of notes on desktop app since that will be the first app that's released most likely.
- [ ] Move over rest of components below:
- [ ] Work on remaining parsers

---

## Missing or Incomplete Parsers

- [x] CodeBlock
  - [x] Check for meta string without `--` and alert user.
- [x] Footnote
- [?] Table (GFM if it's not a performance hit)
- [ ] Strikethrough text
- [ ] Escapable math blocks.
- [ ] Escapable strings
- [x] Paragraphs (reimplemented)
- [ ] List Item
  - [ ] With various checked states: `[?]`, `[x]`, `[-]`, `[ ]`, `[$stateVariable]`, etc...
  - [ ] Add interactive List component that allows the user to create interactive lists, but outputs the content to markdown when rendered.
    - [ ] This would require the start of a `conundrum_context` crate.
- [ ] `[My link](myId@10:30:00)` timestamp link syntax.

### Way off in the distance

- [ ] Go through docs and implement template strings for all Rust based properties. Move all documentation to the docgen crate.
  - [ ] Checkbox checked states
  - [ ] Emphasis with emphasis default previews

---

# Paper

## Add to paper

- [ ] The derivation of the following, and the significance.

$$
\frac{dx}{x} = \frac{dr}{R}
$$

- [ ] The following as a form of spatal tension giving rise to $\alpha$

$$
2 \frac{R_{\oplus}}{\left( 2 G M_{\oplus} \right)^{1/3}} = \frac{1}{\alpha}
$$
