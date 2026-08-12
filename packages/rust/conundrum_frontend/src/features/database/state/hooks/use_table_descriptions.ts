import { rspc } from "@/app/rspc_client";

export const useTableDescriptions = () => {
    return rspc.useQuery(["describe.all_tables", null], {});
};
