import { type Procedures } from "@/codegen/bindings";

export type AgentDescription =
    Procedures["crud"]["agent_description"]["get_by_predicate"]["output"][number];
