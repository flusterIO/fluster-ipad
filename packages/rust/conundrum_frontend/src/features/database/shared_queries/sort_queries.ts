import { type SortQuery } from "@/codegen/bindings";

export const sortByCtime: SortQuery = {
    column: "ctime",
    order: "desc-null-last",
};
