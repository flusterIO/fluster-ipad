import { sizeableSchema } from "../../embeddable_component_utils/schemas/embeddable_component_schemas"

export const admonitionPropsSchemaGetter = () => {
    return sizeableSchema({
        booleanClasses: {
            left: "float-left",
            right: "float-right",
            center: "mx-auto"
        },
        width: {
            sizeClasses: {
                small: "w-full @5xl/mdx:max-w-60",
                medium: "w-full @5xl/mdx:max-w-120",
                large: "w-full @5xl/mdx:max-w-180",
            }
        },
        margin: {
            sizeClasses: {
                none: "",
                large: "m-8",
                medium: "m-4",
                small: "m-2",
            }
        }
    })
}
