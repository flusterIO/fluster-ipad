import { describe, expect, it } from 'vitest';
import { TestInputItem } from '../../../../development/test_types';
import { zodSchemaSources } from "./zod_schema_sources";
import { ZodToMarkdownHandler } from './zod_to_markdown_handler';



describe('Parses zod schemas and returns valid markdown strings', () => {
    const input: Pick<TestInputItem<InstanceType<typeof ZodToMarkdownHandler>["zodSchemaToMarkdown"]>, "input">[] = [
        ...zodSchemaSources.map((s) => {
            return {
                input: [s.schema] as [typeof s.schema]
            }
        })
    ]
    it('Maps over test input and returns proper output', () => {
        for (const testItem of input) {
            const handler = new ZodToMarkdownHandler([])
            const res = handler.zodSchemaToMarkdown(...testItem.input)
            expect(res).toMatchSnapshot()
        }
    });
});



