import { describe, expect, it } from 'vitest';
import { SizableOption } from '../schemas/sizable_props_schema';
import { breakpointBySize, defaultColumnsByBreakSize, getColumns, getSmallestSizableBreakpointByWidth } from './embeddable_responsive_grid_props';
import { TestInputItem } from '../../../../development/test_types';


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
            fit: defaultColumnsByBreakSize.full,
            full: 2,
        },
    },
    {
        input: {},
        expected: {
            ...defaultColumnsByBreakSize,
            fit: defaultColumnsByBreakSize.full
        }
    }
]

describe('getColumns returns the proper next smallest column', () => {
    it('Maps over test input and returns proper output', () => {
        for (const testItem of items) {
            const res = getColumns(testItem.input)
            expect(res).toEqual(testItem.expected)
        }
    });
});



describe('getSmallestSizableBreakpointByWidth returns the proper width', () => {
    const input: TestInputItem<typeof getSmallestSizableBreakpointByWidth>[] = [
        {
            input: [
                0
            ],
            expected: "none"
        },
        ...Object.entries(breakpointBySize).map((bp) => {
            return {
                input: [
                    bp[1]
                ],
                expected: bp[0] as keyof typeof breakpointBySize
            } satisfies TestInputItem<typeof getSmallestSizableBreakpointByWidth>
        })
    ]
    it('Maps over test input and returns proper output', () => {
        for (const testItem of input) {
            const res = getSmallestSizableBreakpointByWidth(...testItem.input)
            expect(breakpointBySize[res!]).toEqual(breakpointBySize[testItem.expected!])
        }
    });
});
