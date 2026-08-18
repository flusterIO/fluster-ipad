import { type Procedures } from "@/codegen/bindings";

export type HealthReport = Procedures["rpc_health"]["output"];

export type BackendStatus = Procedures["backend_status"]["output"];
