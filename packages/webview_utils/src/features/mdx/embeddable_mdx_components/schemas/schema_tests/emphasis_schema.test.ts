import { describe, expect, it } from 'vitest';
import { emphasisForegroundTransform, emphasisToForegroundClasses, EmphasisTransformDefaultKey, ZodStylesGroup } from '../emphasis_schema';



describe('Returns proper emphasis properties from varied css/class input.', () => {
    const input: { input: EmphasisTransformDefaultKey, expected: ZodStylesGroup }[] = [
        {
            input: {
                backgroundColor: "black"
            },
            expected: {
                classes: "",
                css: {
                    backgroundColor: "black"
                }
            }
        },
        {
            input: "important",
            expected: {
                classes: emphasisToForegroundClasses("important"),
                css: {}
            }
        }
    ]
    it('Maps over test input and returns proper output', () => {
        for (const testItem of input) {
            const res = emphasisForegroundTransform(testItem.input)
            expect(res).toMatchSnapshot()
        }
    });
});



