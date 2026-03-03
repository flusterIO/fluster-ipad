import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs"
import { getEmphasisOptions } from "#/mdx/embeddable_mdx_components/schemas/emphasis_schema"
import { stringToSerializedString } from "#/serialization/methods/string_to_serialized_string"
import { faker } from "@faker-js/faker"
import { TestStringUtilities } from "../../../../../development/test_string_utilities"


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
`,
        `
## HR with children

<Hr>As text</Hr>

<Hr>

As paragraph

</Hr>

<Hr>

Hr with really long text because inevitably some user will try this just to see if it works so I need to make this text really long.

</Hr>
`,
        `
## Research Container Classes

<Container research centerContent>
This is my container
</Container>
`,
        `
## Embeddable Hint

<Hint>My text</Hint>

<Hint>

As a paragraph

</Hint>

<Hint>
Cursus quisque imperdiet morbi et quisque dignissim, diam, lectus dignissim. Consequat elit congue molestie rhoncus, lectus condimentum rutrum eget lectus. Parturient leo lectus risus faucibus vitae risus consequat, convallis arcu. Praesent dignissim, eu maecenas porttitor semper, ut convallis dui integer. Iaculis, tristique natoque lorem condimentum libero auctor massa, hac velit.
</Hint>
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


export const getFakeNoteContent = async (paragraphs: number = 10): Promise<string> => {
    const body = faker.lorem.paragraphs(paragraphs).split("\n")
    const su = new TestStringUtilities()

    for (const testItem of testContent()) {
        const index = Math.random() * body.length - 1
        body.splice(index, 0, testItem)
    }

    for await (const cfg of embeddableComponentConfigs) {
        const index = Math.random() * body.length - 1
        const res = await cfg.generateTestContent(faker, su)
        body.splice(index, 0, res)
    }

    return body.join("\n\n")
}

export const loadFakeNote = async () => {
    const body = await getFakeNoteContent()
    const bytes = stringToSerializedString(body)
    window.setEditorContent(body)
    window.setParsedEditorContentString(bytes)
}
