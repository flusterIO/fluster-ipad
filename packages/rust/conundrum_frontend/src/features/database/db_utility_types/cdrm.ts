import { type Procedures } from "@/codegen/bindings";

export type CDRMModelItem =
    Procedures["crud"]["cdrm"]["get_by_predicate"]["output"][number];
