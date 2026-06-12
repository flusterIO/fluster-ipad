# To-Do

## Release, Step-by-Step

- [ ] Push up website update so privacy page is available.
- [ ] Begin upload of youtube videos so that they can parse and validate while you're doing everything else.
- [ ] Figure out how to integrate Apple Pay with the debug build for production.
  - `I'm a pretty experienced developer that's new to the Apple ecosystem. I've already built a complete MacOS application that integrates ApplePay locally for auto-renewing subscriptions. Walk me through the steps required to publish this app today, ensuring that ApplePay will work.`
- [ ] Convert screenshots to mockups
- [ ] Begin submission process with Apple. Upload the build early to make sure that it's valid, incase you need to rebuild it again.
- [ ] Make sure iPad app at least attemps to build while stll on WIFI so everything that's required can be downloaded.
- [ ] Figure out how to call out of loop on another thread in the new editor package so that can hopefully be used with the new apps!

---

### Pre-release bugs

- [x] Verify privacy page on website so it's ready to deploy when you get to the library.
- [x] Make tags clickable again! Redirect to the tags search page.
- [x] Make sure shit bib snippets are not shown.
- [x] Remove section on both the website and the app about $R  \alpha^3 = \text{cubit}$. It's wrong you dumb cunt.
- [x] Work on initial launch note conversion to `.cdrm`
- [x] Initial note seeding
- [x] Onboarding toast things with ToastKit or whatever it's called.
- [?] Front-matter titles cannot be unset! This is a huge problem if people actually decide to use them.

### After Release

- [ ] Reintegrate Dictionary entry! That's a huge value add that's basically done already.
- [ ] Add support for mermaid via mmdflux.
- [ ] Add 'col-span' property from 1-12 to the `Sizable` struct.
- [x] Get `wasm` loading in Next.js. That'll be perfect for enabling a web based editor, but it's also crucial for a decent development workflow now that we're rendering straight to html and a React based test page doesn't make as much sense.
- [x] Make blog mobile responsive. That should be the first priority once conundrum content is compiling again.
  - [x] Add drawer in place of sidebar on mobile.
    - Installed `react-device-detect` for the switch
- [x] Get checkbox parser working
- [ ] Create various state comparison methods to compare the json data to url query strings, so that it can be done server side. Keep all of the blog state that isn't component local in the url.
- [ ] Add 'env-file-provider' to configuration if the user provides a 'env_file' key in their config.

---

## Pre-Release Milestones

- [ ] Get dictionary entry page working (requires dictionary html output parser and new query method to return properly formatted html)
  - [x] Render dictionary entries to html
  - [ ] Handle rendering of dictionary page in React still, directly from the Swift DB.
- [ ] All documentation pages back in working order (Handle tables, list items and rest of missing parsers)
  - [ ] Parse & Render Table
  - [ ] Parse & Render Lists
    - [x] Unordered (half complete)
    - [x] Ordered
    - [x] Checkbox
  - [x] Parse & Render Footnotes (half complete)
  - [ ] Emoji search and other doc specific components working as expected.
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)

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
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)

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
