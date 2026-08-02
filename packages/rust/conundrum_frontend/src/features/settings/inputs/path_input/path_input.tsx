import React, { type ReactNode } from "react";
import { type LabeledImportProps } from "../general_input_props";
import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import { type Path, type FieldValues } from "react-hook-form";

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
    mustExist,
    isDirPath,
    label,
    form,
    name,
    desc,
}: PathInputProps<Schema>): ReactNode => {
    console.log("mustExist, isDirPath: ", mustExist, isDirPath);
    const value = form.watch(name);
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
            {desc ? <div className="text-sm text-foreground/80!">{desc}</div> : null}
        </div>
    );
};

PathInput.displayName = "PathInput";
