public enum IpadMainViewTab: String, Sendable {
  case paper,
    markdown,
    bib,
    noteDetail,
    notes,
    settings,
    createNote,
    searchNotes,
    /// The "Notes" row in the sidebar's Search section. Must differ from `searchNotes`, which is
    /// the pinned search tab; two tabs sharing one selection value makes selection unreliable.
    searchNotesList,
    searchByBib,
    searchByTag,
    searchByTopic,
    searchBySubject,
    bookmarks,
    dictionary
}
