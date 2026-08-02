import React, { useEffect, useEffectEvent, type ReactNode } from "react";
import { type LabeledImportProps } from "../general_input_props";
import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import { type Path, type FieldValues, type ErrorOption } from "react-hook-form";
import { FormMessage } from "@/components/shad/form";
import { pathExists } from "#/file_system/path_utils/path_exists";

interface PathInputProps<
    Schema extends FieldValues,
> extends LabeledImportProps<Schema> {
    /**
     * Set to true if the intended path is meant to be a directory.
     */
    isDirPath?: boolean;
    /**
     * Defaults to true
     */
    mustExist?: boolean;
}

export const PathInput = <Schema extends FieldValues>({
    isDirPath,
    label,
    form,
    name,
    desc,
}: PathInputProps<Schema>): ReactNode => {
    const value = form.watch(name);
    const setPathNotExistError = (): void => {
        form.setError(name, {
            message: "This path does not exist.",
        } satisfies ErrorOption);
    };
    const pathDoesExist = useEffectEvent(async (fp: string) => {
        const res = await pathExists(fp);
        if (!res) {
            setPathNotExistError();
        } else {
            form.clearErrors(name);
        }
    });
    useEffect(() => {
        if (value !== "") {
            pathDoesExist(value);
        } else if (form.getFieldState(name).isTouched) {
            setPathNotExistError();
        }
    }, [value]);
    return (
        <div className="flex flex-col justify-center items-start gap-y-2">
            <Label>{label}</Label>
            <Input
                value={value}
                onChange={(e) => {
                    // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
                    form.setValue(name, e.target.value as any);
                }}
            />
            <FormMessage>
                {desc ? (
                    <div className="text-sm text-foreground/80!">{desc}</div>
                ) : null}
            </FormMessage>
        </div>
    );
};

PathInput.displayName = "PathInput";
