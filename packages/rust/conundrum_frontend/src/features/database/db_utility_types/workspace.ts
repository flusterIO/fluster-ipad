import { type Procedures } from "@/codegen/bindings";

export type WorkspaceUpdateRequest =
    Procedures["crud"]["user_workspace"]["update_many"]["input"];

export type WorkspaceByPredicate =
    Procedures["crud"]["user_workspace"]["get_by_predicate"]["output"][number];
