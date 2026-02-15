
import { SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { Completion, snippetCompletion } from "@codemirror/autocomplete";


// This might not be needed as all settings can be set in the app itself now.
export const getBibtexSnippets = (): {
    entries: Completion[]
    fields: Completion[]
} => {
    return {
        entries: [
            snippetCompletion(`@misc{#{someUniqueKeyThatNeverChanges},
\tauthor = {#{Author Name}},
\ttitle = {#{Page Title}},
\tyear = {#{Year}},
\turl = {#{https://flusterapp.com}},
\tnote = {#{Any useful information}}
}`, {

                label: "@website",
                detail: "BibTeX website entry",
                type: SnippetDefaultType.text,
                // section: COmpl
            }),
        ],
        fields: []
    }
}
