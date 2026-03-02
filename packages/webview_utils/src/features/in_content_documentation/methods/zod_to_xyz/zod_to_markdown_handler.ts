import assert from "node:assert"
import { marked } from 'marked';
import { markedTerminal } from 'marked-terminal';
import { z } from "zod"


/* @ts-expect-error -- Just a useless development tool */
marked.use(markedTerminal({
}, { jsx: true }));

export class ZodToMarkdownHandler {
    body: string
    ignoreKeys: string[]
    constructor(ignoreKeys: string[], fileHeader: string = "") {
        this.body = fileHeader
        this.ignoreKeys = ignoreKeys
    }


    logBoolean(optional: boolean | null = null) {
        this.body += `\n**Boolean${optional === true ? " (optional)" : ""}**\n`
    }

    logZodEnum(values: (string | number)[], optional: boolean | null = null) {
        let s = `\n**Enum${optional === true ? " (optional)" : ""}:** One of the following:\n\n`
        assert(Array.isArray(values), "Found an enum without values")
        for (const option of values!) {
            assert(["string", "number"].includes(typeof option), "Found a zod venum value that isn't a valid type.")
            s += `- ${option}\n`
        }
        this.body += s
    }


    logNumber(data: z.ZodNumberDef) {
        this.body += `\n**Number**`
        console.log("data: ", data)
        if (data.description) {
            this.body += ` ${data.description}\n\n`
        } else {
            this.body += "\n"
        }
        if (data.coerce) {
            this.body += "- coerced (Will help you convert strings to numbers)"
        }
    }


    /* eslint-disable-next-line  -- Need to use any here. */
    logZodUnion(itemType: z.ZodUnion<any>, optional: boolean | null = null) {
        this.body += `\n**Union${optional === true ? " (optional)" : ""}: One of the following types**\n`
        /* eslint-disable-next-line  -- Need to use any here. */
        itemType.options.forEach((opt: any, i: number, a: any[]) => {
            this.anyZodToMarkdown(opt)
            if (i < a.length - 1) {
                this.body += this.logSeperator()
            }
        })
    }



    async logMarkdown(content: string): Promise<void> {
        // const renderer = markedTerminal({}, { jsx: true })
        // const res = await parseInline(content)
        console.log(marked.parse(content))
    }

    logPropertyKey(k: string) {
        this.body += `\n#### ${k}\n\n`
    }

    logZodObject(item: z.ZodObject<z.ZodRawShape>) {
        // const s = ""

        const shape = item._def.shape()
        const itemKeys = Object.keys(shape).filter((k) => !this.ignoreKeys.includes(k))
        for (const itemKey of itemKeys) {
            const itemData = shape[itemKey]
            this.logPropertyKey(itemKey)
            this.anyZodToMarkdown(itemData)
            //                 s += `
            // #### ${itemKey}
            // ${this.anyZodToMarkdown(itemType)}

            // ${description ?? ""}

            // `
        }
        // this.body += s
    }

    logString(item: z.ZodStringDef, optional: boolean | null = null) {
        this.body += `\n**String${optional === true ? " (optional)" : ""}**`
        if (item.description) {
            this.body += ` ${item.description}\n\n`
        } else {
            this.body += "\n"
        }
    }

    logSeperator(seperatorContent: string = "Or") {
        this.body += `\n<Hr>${seperatorContent}</Hr>\n`
    }
    /* eslint-disable-next-line  -- Need to use any here. */
    anyZodToMarkdown(itemType: any, optional: boolean | null = null): void {
        console.log("itemType instanceof ZodEnum: ", itemType instanceof z.ZodObject)
        if (!itemType) {
            console.error("Called anyZodToMarkdown witha nullish type.")
        }
        if (itemType instanceof z.ZodObject) {
            return this.logZodObject(itemType)
        }
        if (itemType instanceof z.ZodEffects) {
            console.count(`Returning ZodEffects?`)
            return this.anyZodToMarkdown(itemType.innerType())
        }
        if (itemType instanceof z.ZodBoolean || itemType?.typeName === "ZodBoolean") {
            return this.logBoolean(optional)
        }
        if (itemType?.typeName === "ZodEnum") {
            return this.logZodEnum(itemType.values!, optional)
        }
        if (itemType instanceof z.ZodEnum) {
            return this.logZodEnum(itemType._def.values, optional)
        }
        if (itemType instanceof z.ZodUnion) {
            return this.logZodUnion(itemType, optional)
        }
        if (itemType instanceof z.ZodOptional) {
            return this.anyZodToMarkdown(itemType._def.innerType, true)
        }
        if (itemType instanceof z.ZodNumber) {
            return this.logNumber(itemType._def)
        }
        if (itemType instanceof z.ZodString) {
            return this.logString(itemType._def)
        }
        if (!itemType?.typeName) {
            console.log("Without type: ", itemType)
            throw new Error(`Found a zod value without a vald type`)
        }

        // if (itemType.typeName === "ZodBoolean") {
        //     return this.logBoolean()
        // }
        // if (itemType.typeName === "ZodEnum") {
        //     return this.logZodEnum(itemType)
        // }

        // if (itemType.options) {
        //     return this.logZodUnion(itemType as WeirdZodType & Required<Pick<WeirdZodType, "options">>)
        // }

        console.warn("itemType reached here: ", itemType)
        throw new Error("Found a Zod object that was not handled properly.")
    }
    /**
     * For use in development only. This this is likely going to be super unreliable.
     */
    zodSchemaToMarkdown(
        schema: z.ZodTypeAny,
        /**
         * Required to ignore specifc keys like the error trigger property
         * used in the sizable model.
         */
        // ignoreKeys: string[],
    ): string {
        const s = this.body
        this.anyZodToMarkdown(schema)

        this.logMarkdown(this.body)
        return s
    }
}
