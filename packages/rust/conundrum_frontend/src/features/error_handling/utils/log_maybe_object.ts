import consola from "consola";

const formatObj = (obj: any): string => {
    if (typeof obj === "string") {
        return obj;
    }
    if (typeof obj === "object" && !Array.isArray(obj)) {
        return JSON.stringify(obj, null, 2);
    }
    return `${obj}`;
};

export const logMaybeObject = (
    prefix: string,
    obj: any,
    variant: "log" | "info" | "error" | "warn" = "log",
) => {
    consola[variant](`${prefix}${formatObj(obj ?? {})}`);
};
