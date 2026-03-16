import { embeddableComponentConfigs } from "../../features/mdx/embeddable_mdx_components/component_configs";
import { specialSyntaxItems } from "../../features/syntax/data/syntax_items";
import { faker } from "@faker-js/faker";
import { TestStringUtilities } from "../test_string_utilities";
import { type EmbeddableComponentConfig } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { type FlusterSyntax } from "#/syntax/data/syntax_types";

export const standardBenchmarkLengths = [5, 10, 20, 50];

/**
 * Don't touch this function any more. Use this as a standardized way to generate benchmark content at varying lengths.
 */
export const getBenchmarkContent = async (
    /**
     * The number of admonitions to include,
     * since admonitions have a testProps.quantityScalar = 1
     */
    length = 20,
) => {
    const stringUtils = new TestStringUtilities();

    const paragraphs = Array(length * 3)
        .fill(0)
        .map(() => {
            return faker.lorem.paragraphs(1);
        });
    const insertRandomly = (value: string, inline: boolean): void => {
        if (inline) {
            const idx = Math.floor(Math.random() * paragraphs.length)
            const paragraphText = paragraphs[idx]
            const pSplit = paragraphText.split(" ")
            pSplit.splice(
                Math.floor(Math.random() * pSplit.length),
                0,
                value,
            );
            paragraphs[idx] = pSplit.join(" ")
        } else {
            paragraphs.splice(
                Math.floor(Math.random() * paragraphs.length - 1),
                0,
                value,
            );
        }
    };
    const inlineComponents: EmbeddableComponentConfig[] = []
    const blockComponents: EmbeddableComponentConfig[] = []
    const inlineSyntaxes: FlusterSyntax[] = []
    const blockSyntaxes: FlusterSyntax[] = []
    specialSyntaxItems.forEach((c) => {
        if (c.isInline) {
            inlineSyntaxes.push(c)
        } else {
            blockSyntaxes.push(c)
        }
    })
    embeddableComponentConfigs.forEach((c) => {
        if (c.isInline) {
            inlineComponents.push(c)
        } else {
            blockComponents.push(c)
        }
    })


    for (const syntax of inlineSyntaxes) {
        const quantityOfComponent = Math.floor(Math.random() * length * syntax.testQuantityScalar);
        for (const _ of Array(quantityOfComponent).fill(0)) {
            const s = await syntax.testContentGenerator(faker);
            insertRandomly(s, syntax.isInline);
        }
    }


    for (const cmp of inlineComponents) {
        const quantityOfComponent = Math.floor(Math.random() * length * 2 * cmp.testProps.quantityScalar);
        for (const _ of Array(quantityOfComponent).fill(0)) {
            const s = await cmp.generateTestContent(faker, stringUtils);
            insertRandomly(s, cmp.isInline);
        }
    }


    for (const syntax of blockSyntaxes) {
        const quantityOfComponent = Math.floor(Math.random() * length * syntax.testQuantityScalar);
        for (const _ of Array(quantityOfComponent).fill(0)) {
            const s = await syntax.testContentGenerator(faker);
            insertRandomly(s, syntax.isInline);
        }
    }


    for (const cmp of blockComponents) {
        const quantityOfComponent = Math.floor(Math.random() * length * 2 * cmp.testProps.quantityScalar);
        for (const s of Array(quantityOfComponent).fill(0)) {
            const s = await cmp.generateTestContent(faker, stringUtils);
            insertRandomly(s, cmp.isInline);
        }
    }


    const content = paragraphs.join("\n\n");
    console.log(content);
    return content;
};
