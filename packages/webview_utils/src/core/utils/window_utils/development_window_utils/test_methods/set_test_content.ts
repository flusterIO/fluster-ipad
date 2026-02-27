import { getEmphasisOptions } from "#/mdx/embeddable_mdx_components/schemas/emphasis_schema"
import { stringToSerializedString } from "#/serialization/methods/string_to_serialized_string"
import { faker } from "@faker-js/faker"


const testContent = (): string[] => {
    const items = [
        `
## Code Block

\`\`\`tsx title="myCode.tsx"
const x = (data: MyType): number => {
    return y / Math.random()
}
\`\`\`
`, `
## Block Quote

> This is my block quote here.
> That continues here.
`
    ]
    for (const emphasis of getEmphasisOptions()) {
        items.push(`

<Admonition title="My ${emphasis} admonition" ${emphasis} foldable>
This is my admonition's body.
</Admonition>
`)
    }
    return items
}


export const getFakeNoteContent = (paragraphs: number = 10): string => {
    const body = faker.lorem.paragraphs(paragraphs).split("\n")

    for (const testItem of testContent()) {
        const index = Math.random() * body.length - 1
        body.splice(index, 0, testItem)
    }

    return body.join("\n\n")
}

export const loadFakeNote = () => {
    const body = getFakeNoteContent()
    const bytes = stringToSerializedString(body)
    window.setEditorContent(body)
    window.setParsedEditorContentString(bytes)
}
