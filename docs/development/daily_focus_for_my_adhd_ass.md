## Daily Focus for My ADHD Assistant

### Summary of To-Do List (Generated: Sunday, May 3rd, 2026 — 6:40 PM CST)

#### **High Priority Tasks (Critical for Release in 2-4 Weeks)**

1. **Complete Table Parsing & Rendering**
   - Implement GFM table parsing
   - Ensure tables render correctly in markdown view
   - Fix table heading row parsing

2. **Fix Markdown View Page Rendering**
   - Resolve tab rendering issues on load

3. **Implement Missing Parsers**
   - List Item with checkbox (various states)
   - Numbered list items
   - Full table support
   - Footnote parser
   - Plain line item parser

4. **Reimplement `window.setBibtexEditorContent`**
   - Ensure buffer operations work correctly

5. **Ensure Summary UI Functionality**
   - Allow users to accept or decline summaries

6. **Get Dictionary Entry Page Working**
   - Implement HTML output parser
   - Ensure React rendering from Swift DB

7. **Ensure All Documentation Pages Work**
   - Tables, lists, footnotes, and other parsers

8. **Move Remaining Components**
   - `AINoteSummary` and other components

9. **Integrate Typst Support**
   - Major value add for offline use

10. **Setup Note Seeding for Desktop App**
    - First app to be released

#### **Next Steps**

- Work on remaining parsers
- Move over rest of components
- Handle scroll restoration on desktop app
- Fix iPad initial load issue in portrait mode

#### **Bug Fixes**

- Ensure bibliography entries are properly associated with notes
- Fix 'paper' button creating duplicate pages
- Resolve error state reset on iPad after manual save

#### **AI & Features**

- Implement AI triggers (`auto`, `confirm`, `never`)
- Create `FlusterAI.generateStudyGuide()` and `FlusterAI.createFlashCards()`
- Handle light mode for `AISummaryContainer`
- Allow force regeneration of summaries

#### **Performance & Offline**

- Move regex queries to `HashMap<String, bool>`
- Fix syntax for italic fields in titles
- Pipe errors to webview error display
- Create `SiaString` class for string compression

#### **Release Checklist**

- Ensure all tests pass (Swift, Typescript, Rust)
- Review Apple's nomination process for featured status
- Handle AI availability states
- Generate R3 vectors and note summaries in background

#### **Additional Notes**

- Ensure note summary is applied to new summary class if present
- Fix initial note refresh issues
- Handle new line items not working on initial load
- Work on global search page for Mac

This summary prioritizes tasks that will directly impact the app's release and user experience. Focus on completing these in the next 2-4 weeks to ensure a successful launch.