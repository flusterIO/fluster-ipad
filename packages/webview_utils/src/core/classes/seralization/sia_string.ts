import { addAsciiN, Buffer, readAsciiN } from "@timeleap/sia"

/**
 * What's the opposite of deprecated? Like 'not yet to be precated'? That's what this is... It's not important enough to prioritize but I'm still going to leave it around because I don't have internet to reinstall things later.
 */
export class SiaString {
    val: string
    constructor(val: string) {
        this.val = val
    }

    // BUG: This allocation **will** cause issues if this is ever used for a string of an unknown size.
    serialize(alloc = 1024): Uint8Array {
        const buf = Buffer.alloc(alloc)
        addAsciiN(buf, this.val)
        return buf.toUint8Array()
    }

    static deserialize(data: Uint8Array, length: number): SiaString {
        const buf = new Buffer(data)
        const s = readAsciiN(buf, length)
        return new SiaString(s)
    }
}


