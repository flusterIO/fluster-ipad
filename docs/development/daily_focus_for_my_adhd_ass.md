# Daily Focus for My ADHD-Ass

## Summary of To-Do List (Generated on Friday, August 7th, 2026 — 8:55 PM CST)

### High Priority Tasks

1. **Review and Validate Equations**
   - Check the inserted alpha expansion equations for correctness.
   - Ensure the orbital velocity equation and its derivation are correctly included in the notebook.

2. **iPad App Enhancements**
   - Update the settings page to include light and dark mode codeblock themes.
   - Implement a keymap listener for manual save requests.
   - Ensure notes re-render on theme changes and when users navigate back to them.
   - Remove references to the dictionary page.
   - Add a paywall to the create-note page.
   - Test all features thoroughly.

3. **Plot App Improvements**
   - Create a reset method for the variable enum and a 'reset all' button.

4. **Pre-Release Checklist**
   - Move the 'Download Now' button to the footer and update the title to 'Download old app' or similar.
   - Implement proper links in the website JSON data file.
   - Address the mobile sidebar issue for the blog.

5. **Swift Model Explorer**
   - Move the Constants class to AppStorage and make it static.
   - Save videos on SwiftCharts and Mlx.

6. **Release Preparation**
   - Push the website update to make the privacy page available.
   - Begin uploading YouTube videos for parsing and validation.
   - Integrate Apple Pay with the debug build for production.
   - Convert screenshots to mockups.
   - Start the Apple submission process and upload the build early.
   - Ensure the iPad app can build while on Wi-Fi.
   - Figure out how to call out of loops on another thread in the new editor package.

### Pre-Release Bugs

- Verify the privacy page is ready for deployment.
- Make tags clickable again and redirect to the tags search page.
- Ensure bib snippets are not shown.
- Work on initial launch note conversion to `.cdrm`.
- Seed initial notes.
- Implement onboarding toasts with ToastKit.
- Fix the issue where front-matter titles cannot be unset.
- Add citation for the variable speed of light article to the paper.

### Post-Release Tasks

- Reintegrate the dictionary entry feature.
- Add support for mermaid via mmdflux.
- Add the 'col-span' property to the `Sizable` struct.
- Get `wasm` loading in Next.js for a web-based editor.
- Make the blog mobile responsive with a drawer in place of the sidebar.
- Get the checkbox parser working.
- Create state comparison methods to sync JSON data with URL query strings.
- Add an 'env-file-provider' to the configuration if an `env_file` is provided.

### Pre-Release Milestones

- Ensure the dictionary entry page is working with the HTML parser and React rendering.
- Restore all documentation pages with proper table, list, and footnote parsing.
- Add proper citations to the blogging platform using the citations fluster crate.

### Components to Move

- Container
- HrWithChildren (in JSX form)
- Grid
- AINoteSummary
- Table of Contents (TOC)

### Up Next

- Work on integrating typst support for offline use.
- Set up note seeding for the desktop app.
- Move remaining components and work on parsers.
- Add proper citations to the blogging platform.

### Missing or Incomplete Parsers

- CodeBlock (check for meta strings without `--` and alert user)
- Footnote
- Table (GFM if not a performance hit)
- Strikethrough text
- Escapable math blocks
- Escapable strings
- List Items (with various checked states)
- Interactive List component
- Timestamp link syntax `[My link](myId@10:30:00)`

### Long-Term Goals

- Implement template strings for all Rust properties and move documentation to the docgen crate.
- Checkbox checked states
- Emphasis with default previews

## Paper Additions

- Derive and explain the following equation and its significance:

$$
\frac{dx}{x} = \frac{dr}{R}
$$

- Derive and explain the following as a form of spatial tension giving rise to $\alpha$:

$$
2 \frac{R_{\oplus}}{\left( 2 G M_{\oplus} \right)^{1/3}} = \frac{1}{\alpha}
$$

## Notes

- Prioritize tasks that are critical for release within the next 2-4 weeks.
- Ensure all major features are tested and functioning as expected.
- Keep the focus on completing the most impactful tasks first.
- Regularly review and update the to-do list to reflect progress and new priorities.

---

**Generated on:** Friday, August 7th, 2026 — 8:55 PM CST
**Author:** Andrew
**Project:** Fluster