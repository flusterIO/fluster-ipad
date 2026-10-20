import { sortByUtime } from "#/database/shared_queries/sort_queries";
import { rspc } from "@/app/rspc_client";

export const useRecentNotes = (
    pagination = {
        page: 1,
        per_page: 10,
    },
) => {
    const { data, isLoading } = rspc.useQuery([
        "crud.cdrm.get_by_predicate",
        {
            pagination,
            predicate: null,
            sort: [sortByUtime],
        },
    ]);
    return [data ?? [], isLoading] as [typeof data, boolean];
};
