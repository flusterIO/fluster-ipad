import { getEmphasisOptions } from "../features/mdx/embeddable_mdx_components/schemas/emphasis_schema"

export class TestStringUtilities {
    constructor() { }
    randomBooleanProperty(propertyName: string, propability: number = 0.5) {
        if (Math.random() >= propability) {
            return propertyName
        }
        return ""
    }

    randomBooleanProperties(propertyNames: string[], propability: number = 0.5) {
        return propertyNames.map((n) => this.randomBooleanProperty(n, propability)).filter((f) => f.length).join(" ")
    }
    randomEmphasis() {
        const arr = getEmphasisOptions()
        return arr[Math.floor(Math.random() * arr.length)]
    }

    valueIfRandomProablity<T>(value: T, prob: number = 0.5): T | "" {
        return Math.random() > prob ? value : ""
    }
}
