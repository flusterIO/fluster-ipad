import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs";
import { faker } from "@faker-js/faker";
import { TestStringUtilities } from "../test_string_utilities";

export const standardBenchmarkLengths = [5, 10, 20, 50];

/**
 * Don't touch this function any more. Use this as a standardized way to generate benchmark content at varying lengths.
 */
export const getBenchmarkContent = async (
    /**
     * The number of admonitions to include,
     * since admonitions have a testProps.quantityScalar = 1
     */
    length: number = 20,
) => {
    const stringUtils = new TestStringUtilities();

    const paragraphs = Array(length * 3)
        .fill(0)
        .map((c, i) => {
            return faker.lorem.paragraphs(1);
        });
    const insertRandomly = (value: string): void => {
        paragraphs.splice(
            Math.floor(Math.random() * paragraphs.length - 1),
            0,
            value,
        );
    };
    for await (const config of embeddableComponentConfigs) {
        const quantityOfComponent = Math.floor(Math.random() * length * 2);
        Array(quantityOfComponent)
            .fill(0)
            .forEach(async () => {
                const s = await config.generateTestContent(faker, stringUtils);
                // console.log("s: ", s);
                insertRandomly(s);
            });
    }
    const content = paragraphs.join("\n\n");
    console.log(content);
    return content;
};
