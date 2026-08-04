import { useEffect } from "react";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { type FieldValues, type Path, useFormContext } from "react-hook-form";
import { useFormField } from "@/components/shad/form";
import { type ParsableFileType, type PathSourceType } from "@/codegen/bindings";
import { isShitError } from "#/error_handling/utils/is_shit_error";

/**
 * Returns true if the path is empty in the path sense, being truly empty or the home directory
 */
const pathEmptyKinda = (val: string): bool => {
    const vt = val.trim();
    if (vt.length === 0 || vt === "/" || vt === "\\" || val === "~") {
        return true;
    } else {
        return false;
    }
};

// export const pathExists = async (fp: string): Promise<boolean> => {
//   return await client.query(["fs.path_exists", fp]);
// };

/**
 * Returns null initially before checking.
 */
export const usePathExists = <T extends FieldValues>({
    pathValue,
    name,
    permitted_types = [],
    source_type = "any",
    debounce = 300,
}: {
    pathValue: string;
    name: Path<T>;
    permitted_types?: ParsableFileType[];
    source_type?: PathSourceType;
    debounce?: number;
}): { exists: boolean | null; isLoading: boolean } => {
    const { data, isLoading, error } = rspc.useQuery(
        [
            "fs.validate_path",
            {
                path: pathValue,
                permitted_types,
                source_type,
            },
        ],
        {
            retryDelay: debounce,
        },
    );
    const { setError, clearErrors } = useFormContext<T>();
    const { isTouched } = useFormField();
    useEffect(() => {
        if (error || data === false) {
            if (error) {
                consola.error(`Path Error (${error.code}): ${error.message}`);
            }
            if (typeof pathValue === "string") {
                if (pathEmptyKinda(pathValue)) {
                    clearErrors(name);
                } else {
                    setError(name, {
                        message:
                            !error.message?.length || isShitError(error.message)
                                ? "This path cannot be found on your system."
                                : error.message,
                    });
                }
            }
        } else {
            clearErrors(name);
        }
    }, [error, data, isTouched]);
    return { exists: data ?? null, isLoading };
};
