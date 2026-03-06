import { ParserId } from "@/code_gen/typeshare/fluster_core_utilities";

export interface SpecialSyntax {
    generator: () => string;
    id: ParserId;
}

export const specialSyntaxItems = [

] as const
