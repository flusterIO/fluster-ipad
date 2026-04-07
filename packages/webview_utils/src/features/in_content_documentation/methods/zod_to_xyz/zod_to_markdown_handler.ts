import assert from "node:assert"
import { marked } from 'marked';
import { markedTerminal } from 'marked-terminal';
import { z } from "zod"
import { _sizableObjectSchema } from "../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema";
import { _emphasisSchema } from "../../../mdx/embeddable_mdx_components/schemas/emphasis_schema";
import consola from "consola";


/* @ts-expect-error -- Just a useless development tool */
marked.use(markedTerminal({
}, { jsx: true }));

export class ZodToMarkdownHandler {
    body: string
    ignoreKeys: string[]
    constructor(ignoreKeys: string[], fileHeader = "") {
        this.body = fileHeader
        this.ignoreKeys = ignoreKeys
    }


    logBoolean(optional: boolean | null = null) {
        this.body += `\n**Boolean${optional === true ? " (optional)" : ""}**\n`
    }

    /* eslint-disable-next-line  -- Need to use any here. */
    logZodEnum(values: (string | number)[], optional: boolean | null = null, defaultValue: any | null = null) {
        let s = `\n**Enum${optional === true ? " (optional)" : ""}:** One of the following:\n`
        if (["string", "number"].includes(typeof defaultValue)) {
            s += `**Default:** ${defaultValue}`
        } else {
            s += "\n"
        }
        assert(Array.isArray(values), "Found an enum without values")
        for (const option of values) {
            assert(["string", "number"].includes(typeof option), "Found a zod venum value that isn't a valid type.")
            s += `- ${(option as string | number | undefined) ?? "`undefined`"}\n`
        }
        this.body += s
    }


    logNumber(data: z.ZodNumberDef) {
        this.body += `\n**Number**`
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
    logZodUnion(itemType: z.ZodUnion<any>, optional: boolean | null = null, defaultValue: any | null = null) {
        this.body += `\n**Union${optional === true ? " (optional)" : ""}: One of the following types**\n`
        /* eslint-disable-next-line  -- Need to use any here. */
        itemType.options.forEach((opt: any, i: number, a: any[]) => {
            this.anyZodToMarkdown(opt)
            if (i < a.length - 1) {
                this.body += this.logSeperator()
            }
        })

        if (typeof defaultValue !== "undefined") {
            this.body += `\n**Default:**: ${defaultValue}`
        }
    }

    logDescription(desc: string) {
        this.body += `\n${desc}\n`
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

        const shape = item._def.shape()
        const itemKeys = Object.keys(shape).filter((k) => !this.ignoreKeys.includes(k)).sort((k) => {
            if (k in _sizableObjectSchema || k in _emphasisSchema) {
                return 1
            }
            return -1
        })
        for (const itemKey of itemKeys) {
            const itemData = shape[itemKey]
            console.log("itemData: ", itemData)
            this.logPropertyKey(itemKey)
            if (typeof itemData._def.description === "string") {
                this.logDescription(itemData._def.description as string)
            }
            this.anyZodToMarkdown(itemData)
            //                 s += `
            // #### ${itemKey}
            // ${this.anyZodToMarkdown(itemType)}

            // ${description ?? ""}

            // `
        }
        // this.body += s
    }
    logAny() {
        this.body += "\n**Any**\n"
    }

    logString(item: z.ZodStringDef, optional: boolean | null = null, defaultValue: string | number) {
        this.body += `\n**String${optional === true ? " (optional)" : ""}**`
        if (defaultValue) {
            this.body += ` The default is value "${defaultValue}".\n\n`
        } else {
            this.body += "\n"
        }
        if (item.description) {
            this.body += `- ${item.description}`
        }
    }

    logLiteral(item: string | number, optional: boolean | null = null) {
        this.body += `\n**Literal${optional === true ? " (optional)" : ""}: **, ${item}`
    }

    logSeperator(seperatorContent = "Or") {
        this.body += `\n<Hr>${seperatorContent}</Hr>\n`
    }
    /* eslint-disable-next-line  -- Need to use any here. */
    anyZodToMarkdown(itemType: any, optional: boolean | null = null, defaultValue: any = undefined): void {
        if (!itemType) {
            console.error("Called anyZodToMarkdown witha nullish type.")
        }
        if (itemType instanceof z.ZodEnum) {
            this.logZodEnum(itemType._def.values, optional, defaultValue); return;
        }
        if (itemType instanceof z.ZodString) {
            this.logString(itemType._def, optional, defaultValue); return;
        }

        if (itemType instanceof z.ZodUnion) {
            this.logZodUnion(itemType, optional, defaultValue); return;
        }
        if (defaultValue) {
            console.log("itemType: ", itemType)
            console.warn("defaultValue: ", defaultValue)
            throw new Error(`Found a default value that is being sent to a type that hasn't implemented value to markdown method that handles the default value yet.`)
        }
        if (itemType instanceof z.ZodObject) {
            this.logZodObject(itemType); return;
        }
        if (itemType instanceof z.ZodEffects) {
            this.anyZodToMarkdown(itemType.innerType()); return;
        }
        if (itemType instanceof z.ZodBoolean || itemType?.typeName === "ZodBoolean") {
            this.logBoolean(optional); return;
        }
        if (itemType?.typeName === "ZodEnum") {
            this.logZodEnum(itemType.values, optional); return;
        }
        if (itemType instanceof z.ZodOptional) {
            this.anyZodToMarkdown(itemType._def.innerType, true); return;
        }
        if (itemType instanceof z.ZodNumber) {
            this.logNumber(itemType._def); return;
        }
        if (itemType instanceof z.ZodLiteral) {
            this.logLiteral(itemType._def.value, optional); return;
        }
        if (itemType instanceof z.ZodAny) {
            // Don't log anythng for 'any' yet, always try to guide the user.
            return
            // return this.logAny()
        }
        if (itemType instanceof z.ZodDefault) {
            const d = itemType._def.defaultValue()
            console.log("itemType: ", itemType)

            console.log("d: ", d)
            // assert(typeof d === "object" && Object.keys(d).length === 0, "If this is triggered, there's a default value that isn't an empty object that should probably be documented.")

            if (typeof d === "object" && Object.keys(d).length === 0) {
                return
            } else {
                this.anyZodToMarkdown(itemType._def.innerType, undefined, d); return;
            }
        }
        if (!itemType?.typeName && itemType) {
            consola.error("Zod input without a type: ", itemType)
            // throw new Error(`Found a zod value without a valid type`)
        }

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

        // this.logMarkdown(this.body)
        return s
    }
}
