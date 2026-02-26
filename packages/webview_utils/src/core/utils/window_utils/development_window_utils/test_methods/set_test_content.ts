import { stringToSerializedString } from "#/serialization/methods/string_to_serialized_string"
import { faker } from "@faker-js/faker"


const testContent: string[] = [
    `
# Code Block

\`\`\`tsx title="myCode.tsx"
const x = (data: MyType): number => {
    return y / Math.random()
}
\`\`\`
`, `
# Block Quote

> This is my block quote here.
> That continues here.
`
]


export const getFakeNoteContent = (paragraphs: number = 10): string => {
    const body = faker.lorem.paragraphs(paragraphs).split("\n")

    for (const testItem of testContent) {
        const index = Math.random() * body.length - 1
        body.splice(index, 0, testItem)
    }

    return body.join("\n\n")
}

export const loadFakeNote = () => {
    const body = getFakeNoteContent()
    const bytes = stringToSerializedString(body)
    window.setEditorContent(bytes)
    window.setParsedEditorContentString(bytes)
}
