import { type Procedures } from "@/codegen/bindings";

export type EcosystemLogInput = Procedures["log"]["create"]["input"];
export type EcosystemLogItem = Procedures["log"]["get_many"]["output"][number];
