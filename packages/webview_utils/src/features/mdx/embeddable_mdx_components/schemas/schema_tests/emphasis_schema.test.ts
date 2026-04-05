import { describe, expect, it } from 'vitest';
import { Emphasis, emphasisForegroundTransform, emphasisToForegroundClasses, type EmphasisTransformDefaultKey, type ZodStylesGroup } from '../emphasis_schema';



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
            input: Emphasis.Important,
            expected: {
                classes: emphasisToForegroundClasses(Emphasis.Important),
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



