import { type Procedures } from "@/codegen/bindings";

export type WorkspaceByPredicate =
    Procedures["user_workspace_crud"]["get_by_predicate"]["output"][number];
