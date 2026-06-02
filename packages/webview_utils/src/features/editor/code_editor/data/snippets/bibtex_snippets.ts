import { SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { type Completion, snippetCompletion } from "@codemirror/autocomplete";


// This might not be needed as all settings can be set in the app itself now.
export const getBibtexSnippets = (): Completion[] => {
    return [
        snippetCompletion(`@webpage{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\ttitle = {#{Title}},
\tyear = {#{Year}},
\turl = {#{https://example.com}},
\tnote = {#{Any useful information}}
}`, {

            label: "@website",
            detail: "BibTeX website entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),
        snippetCompletion(`@article{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\ttitle = {#{Title}},
\tyear = {#{Year}},
\turl = {#{https://example.com}},
\tnote = {#{Any useful information}}
}`, {

            label: "@book",
            detail: "BibTeX book entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),
        snippetCompletion(`@book{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\ttitle = {#{Title}},
\tyear = {#{Year}},
\turl = {#{https://example.com}},
\tnote = {#{Any useful information}}
}`, {

            label: "@book",
            detail: "BibTeX book entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),
        snippetCompletion(`@incollection{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\tbooktitle = {#{booktitle}},
\ttitle = {#{Title}},
\tyear = {#{Year}},
\teditor = {#{editor}},
\tpublisher = {#{publisher}},
\turl = {#{https://example.com}},
\tnote = {#{Any useful information}}
\tdate = {#{year}},
\tpages = {#{pages}}
}`, {
            label: "@incollection",
            detail: "BibTeX book entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),

        snippetCompletion(`@techreport{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{author}},
\ttitle = {#{title}},
\tinstitution = {#{institution}},
\tdate = {#{year}},
\tnumber = {#{number}}
}`, {
            label: "@techreport",
            detail: "BibTeX tech report entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),

        snippetCompletion(`@manual{#{someUniqueKeyThatNeverChanges},
\ttitle = {#{title}},
\tauthor = {#{author}},
\torganization = {#{organization}},
\tdate = {#{year}}
}`, {
            label: "@manual",
            detail: "BibTeX manual entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),

        snippetCompletion(`@misc{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\ttitle = {#{Title}},
\tyear = {#{Year}},
\turl = {#{https://example.com}},
\tnote = {#{Any useful information}}
}`, {

            label: "@misc",
            detail: "BibTeX random entry",
            type: SnippetDefaultType.text,
            section: "Entry"
        }),
    ]
}
