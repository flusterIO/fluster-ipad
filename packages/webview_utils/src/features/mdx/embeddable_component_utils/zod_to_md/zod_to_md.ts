import { ZodObject, ZodRawShape } from "zod";

export type ComponentPropsFunctionGetter = <T extends ZodRawShape>() => ZodObject<T>
