import { type SortQuery } from "@/codegen/bindings";

export const sortByCtime: SortQuery = {
    column: "ctime",
    order: "desc-null-last",
};

export const sortByUtime: SortQuery = {
    column: "utime",
    order: "desc-null-last",
};
