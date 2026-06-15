## Daily Focus for My ADHD-Ass

### Summary of To-Do List (Generated on: Sunday, June 14th, 2026 — 7:12 PM CST)

#### 🚀 Release, Step-by-Step

1. **Integrate Apple Pay with the debug build for production**
   - Walk through the steps required to publish the app today, ensuring Apple Pay will work.

2. **Convert screenshots to mockups**

3. **Begin submission process with Apple**
   - Upload the build early to ensure it's valid, in case a rebuild is needed.

4. **Make sure iPad app at least attempts to build while still on WIFI**

5. **Figure out how to call out of loop on another thread in the new editor package**
   - This can be used with the new apps.

#### 🐞 Pre-release bugs

- Ensure privacy page on website is ready to deploy.
- Make tags clickable again, redirecting to the tags search page.
- Ensure bib snippets are not shown.
- Work on initial launch note conversion to `.cdrm`.
- Initial note seeding.
- Onboarding toast things with ToastKit or whatever it's called.
- Add citation for variable speed of light article to paper for release.

#### 📅 After Release

- Reintegrate Dictionary entry (a huge value add already done).
- Add support for mermaid via mmdflux.
- Add 'col-span' property from 1-12 to the `Sizable` struct.
- Get `wasm` loading in Next.js for a web-based editor and development workflow.
- Make blog mobile responsive, starting with a drawer in place of the sidebar on mobile.
- Get checkbox parser working.
- Create various state comparison methods to compare JSON data to URL query strings, keeping blog state in the URL.
- Add 'env-file-provider' to configuration if the user provides an 'env_file' key.

#### 📌 Pre-Release Milestones

- Get dictionary entry page working (requires HTML output parser and new query method).
- All documentation pages back in working order (tables, lists, footnotes, etc.).
- Add proper citations to the blogging platform using the citations fluster crate.

#### 🧩 Components that need to move over still

- Container
- HrWithChildren (in JSX form)
- Grid (not all properties moved)
- AINoteSummary

#### 📌 Then Add

- Table of Contents (TOC)

#### 🧠 Up Next

- Work on integrating typst support (a major value add that can be handled offline and in time for release).
- Setup seeding of notes on the desktop app since it will be the first app released.
- Move over the rest of the components.
- Work on remaining parsers.
- Add proper citations to the blogging platform using the citations fluster crate.

#### 🧩 Missing or Incomplete Parsers

- CodeBlock (check for meta string without `--` and alert user).
- Footnote
- Table (GFM if it's not a performance hit)
- Strikethrough text
- Escapable math blocks
- Escapable strings
- Paragraphs (reimplemented)
- List Item (with various checked states: `[?]`, `[x]`, `[-]`, `[ ]`, `[$stateVariable]`, etc.)
- Interactive List component that allows the user to create interactive lists, but outputs the content to markdown when rendered.
- `[My link](myId@10:30:00)` timestamp link syntax.

#### 🚀 Way off in the distance

- Go through docs and implement template strings for all Rust-based properties. Move all documentation to the docgen crate.
- Checkbox checked states
- Emphasis with emphasis default previews

### 📚 Paper

#### Add to paper

- Derivation of the following and its significance:

$$
\frac{dx}{x} = \frac{dr}{R}
$$

- The following as a form of spatial tension giving rise to $\alpha$:

$$
2 \frac{R_{\oplus}}{\left( 2 G M_{\oplus} \right)^{1/3}} = \frac{1}{\alpha}
$$

### 📌 Final Notes

This summary is prioritized to ensure we can release the application within the next 2-4 weeks. Focus on the most critical tasks first, and tackle the rest as time permits. Stay focused, and keep the ADHD-ass in check!