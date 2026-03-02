

/* eslint-disable-next-line  -- Need to use any her. */
type TestFunction = (...args: any) => any;

export interface TestInputItem<T extends TestFunction> {
    input: Parameters<T>
    expected: ReturnType<T>
}
