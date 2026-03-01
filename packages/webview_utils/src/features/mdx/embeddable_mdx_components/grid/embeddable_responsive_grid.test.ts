import { describe, expect, it } from 'vitest';
import { SizableOption } from '../schemas/sizable_props_schema';
import { defaultColumnsByBreakSize, getColumns } from './embeddable_responsive_grid_props';


interface TestItem {
    input: {
        [K in SizableOption]?: string | number | undefined
    },
    expected: {
        [K in SizableOption]: number | string
    }
}

const items: TestItem[] = [
    {
        input: {
            none: 1,
            medium: 1,
            large: 2,
        },
        expected: {
            none: 1,
            small: 1,
            smedium: 1,
            medium: 1,
            large: 2,
            xl: 2,
            xxl: 2,
            fit: defaultColumnsByBreakSize.fit,
            full: 2,
        }
    }
]

describe('getColumns returns the proper next smallest column', () => {
    it('should add two numbers correctly', () => {
        for (const testItem of items) {
            const res = getColumns(testItem.input)
            expect(res).toEqual(testItem.expected)
        }
    });
});
