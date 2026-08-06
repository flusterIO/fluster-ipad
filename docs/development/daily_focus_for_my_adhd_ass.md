# Daily Focus for My ADHD-Ass

## Summary of To-Do List (Generated on Thursday, August 6th, 2026 — 11:41 AM CST)

### 📌 Prioritized Tasks

#### ✅ Critical Release Readiness

- [x] Push up website update so privacy page is available.
- [x] Begin upload of youtube videos so that they can parse and validate while you're doing everything else.
- [x] Convert screenshots to mockups
- [x] Make blog mobile responsive. That should be the first priority once conundrum content is compiling again.
  - [x] Add drawer in place of sidebar on mobile.
- [x] Get `wasm` loading in Next.js. That'll be perfect for enabling a web based editor, but it's also crucial for a decent development workflow now that we're rendering straight to html and a React based test page doesn't make as much sense.
- [x] Get checkbox parser working
- [x] Make sure iPad app at least attemps to build while still on WIFI so everything that's required can be downloaded.
- [x] Figure out how to call out of loop on another thread in the new editor package so that can hopefully be used with the new apps!
- [x] Reintegrate Dictionary entry! That's a huge value add that's basically done already.
- [x] Add support for mermaid via mmdflux.
- [x] Add 'col-span' property from 1-12 to the `Sizable` struct.
- [x] Add 'env-file-provider' to configuration if the user provides a 'env_file' key in their config.

#### 🚀 Apple App Store Submission

- [x] Begin submission process with Apple. Upload the build early to make sure that it's valid, in case you need to rebuild it again.
- [ ] Figure out how to integrate Apple Pay with the debug build for production.
  - `I'm a pretty experienced developer that's new to the Apple ecosystem. I've already built a complete MacOS application that integrates ApplePay locally for auto-renewing subscriptions. Walk me through the steps required to publish this app today, ensuring that ApplePay will work.`
- [ ] Make sure iPad app at least attemps to build while still on WIFI so everything that's required can be downloaded.

#### 📚 Documentation & Parsing

- [x] Parse & Render Table
- [x] Parse & Render Lists
  - [x] Unordered (half complete)
  - [x] Ordered
  - [x] Checkbox
- [x] Parse & Render Footnotes (half complete)
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)
- [ ] Work on remaining parsers
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)

### 🧠 Additional Notes

- [ ] Work on integrating typst support! That's a **major** value add that can actually be handled offline, and in time to release!
- [ ] Setup seeding of notes on desktop app since that will be the first app that's released most likely.
- [ ] Move over rest of components below:
- [ ] Work on remaining parsers
- [ ] Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conundrum)

### 📚 Missing or Incomplete Parsers

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

### 📖 Paper Additions

- [ ] The derivation of the following, and the significance.

$$
\frac{dx}{x} = \frac{dr}{R}
$$

- [ ] The following as a form of spatal tension giving rise to $\alpha$

$$
2 \frac{R_{\oplus}}{\left( 2 G M_{\oplus} \right)^{1/3}} = \frac{1}{\alpha}
$$

## 📌 Next Steps

- Work on integrating typst support! That's a **major** value add that can actually be handled offline, and in time to release!
- Setup seeding of notes on desktop app since that will be the first app that's released most likely.
- Move over rest of components below:
- Work on remaining parsers
- Add proper citations to blogging platform, with formatted output using the citations fluster crate (Just rename the thing and move it to conund >>