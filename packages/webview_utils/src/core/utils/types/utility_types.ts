import { ReactNode } from "react"

export type WithChildren = {
    children: ReactNode
}


export type MutableArray<T> = T extends ReadonlyArray<infer U> ? U[] : Array<T[keyof T]>;


export type Mutable<T> = { -readonly [K in keyof T]: T[K] };

export type KeysOfType<T, J> = Exclude<{
    [K in keyof T]: NonNullable<T[K]> extends J ? K : never
}[keyof T], undefined>;

export type DeepPartial<T> = T extends object ? {
    [P in keyof T]?: DeepPartial<T[P]>;
} : T;
