import { stringToSerializedString } from "#/serialization/methods/string_to_serialized_string"
import { faker } from "@faker-js/faker"

export const loadFakeNote = () => {
    const body = faker.lorem.paragraphs(10).replaceAll("\n", "\n\n")
    const bytes = stringToSerializedString(body)
    window.setEditorContent(bytes)
    window.setParsedEditorContentString(bytes)
}
