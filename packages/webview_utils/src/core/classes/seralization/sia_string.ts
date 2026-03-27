import { addString64, Buffer, readString64 } from "@timeleap/sia"
export class SiaString {
    val: string
    constructor(val: string) {
        this.val = val
    }

    serialize(alloc = 256): Uint8Array {
        const buf = Buffer.alloc(alloc)
        addString64(buf, this.val)
        return buf.toUint8Array()
    }

    static deserialize(data: Uint8Array): SiaString {
        const buf = new Buffer(data)
        const s = readString64(buf)
        return new SiaString(s)
    }
}


