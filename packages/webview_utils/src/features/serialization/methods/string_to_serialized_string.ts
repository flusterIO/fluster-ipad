import { SerializedString } from "@/code_gen/flat_buffer/shared-webview-data"
import { Builder } from "flatbuffers"

export const stringToSerializedString = (s: string): Uint8Array => {
    let builder = new Builder(1024)
    const bodyOffset = builder.createString(s)
    SerializedString.startSerializedString(builder)
    SerializedString.addBody(builder, bodyOffset)
    const serializedOffset = SerializedString.endSerializedString(builder)
    builder.finish(serializedOffset)
    return builder.asUint8Array()
}
