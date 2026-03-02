import { describe, expect, it } from 'vitest';
import { TestInputItem } from '../../../../development/test_types';
import { zodSchemaToMarkdown } from './zod_to_markdown';
import { zodSchemaSources } from "./zod_schema_sources";



describe('getColumns returns the proper next smallest column', () => {
    const input: Pick<TestInputItem<typeof zodSchemaToMarkdown>, "input">[] = [
        ...zodSchemaSources.map((s) => {
            return {
                input: [s.schema, s.ignore] as [typeof s.schema, typeof s.ignore]
            }
        })
    ]
    it('Maps over test input and returns proper output', () => {
        for (const testItem of input) {
            const res = zodSchemaToMarkdown(...testItem.input)
            expect(res).toMatchSnapshot()
        }
    });
});



