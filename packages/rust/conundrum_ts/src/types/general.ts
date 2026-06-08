import {
    type NextJsConundrumOutput,
    type BlogFileSummary,
} from "../code_gen/typeshare/conundrum";

export type WithNullableOptionals<T> = {
    [K in keyof T]-?: undefined extends T[K]
    ? Exclude<T[K], undefined> | null
    : T[K];
};

export type AnyBuilderOutput = WithNullableOptionals<NextJsConundrumOutput>;
export type AnyNoteOutput = WithNullableOptionals<BlogFileSummary>;
