import React, { useEffect, useEffectEvent, type ReactNode } from "react";
import { type LabeledImportProps } from "../general_input_props";
import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import {
    type FieldValues,
    type ErrorOption,
    useFormState,
    useFormContext,
} from "react-hook-form";
import { FormField, FormMessage, useFormField } from "@/components/shad/form";
import { cn } from "@/utils/shad_utils";
import { usePathExists } from "#/file_system/state/hooks/path_utils/use_path_exists";

interface PathInputProps<Schema extends FieldValues>
    extends
    LabeledImportProps<Schema>,
    Pick<
        Parameters<typeof usePathExists>[0],
        "source_type" | "permitted_types"
    > {
    /**
     * Set to true if the intended path is meant to be a directory.
     */
    isDirPath?: boolean;
    /**
     * Defaults to true
     */
    mustExist?: boolean;
    classes?: {
        container?: string;
        input?: string;
    };
}

const PI = <Schema extends FieldValues>({
    name,
    className,
    source_type = "any",
    /**
     * An empty array of permitted types will allow *any* file type, not just any parsable filetype.
     */
    permitted_types = [],
}: Pick<PathInputProps<Schema>, "name" | "source_type" | "permitted_types"> & {
    className?: string;
}): ReactNode => {
    const form = useFormContext<Schema>();
    const value = form.watch(name);
    usePathExists({
        pathValue: value ?? "/",
        name,
        source_type,
        permitted_types,
    });
    return (
        <Input
            className={cn("text-sm font-mono", className)}
            value={value}
            onChange={(e) => {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
                form.setValue(name, e.target.value as any);
            }}
        />
    );
};

export const PathInput = <Schema extends FieldValues>({
    label,
    form,
    name,
    desc,
    classes = {},
    ...props
}: PathInputProps<Schema>): ReactNode => {
    return (
        <FormField
            name={name}
            render={() => {
                return (
                    <div
                        className={cn(
                            "flex flex-col justify-center items-start gap-y-2",
                            classes.container,
                        )}
                    >
                        <Label>{label}</Label>
                        <PI {...props} name={name} />
                        <FormMessage>
                            {desc ? (
                                <div className="text-sm text-foreground/80!">{desc}</div>
                            ) : null}
                        </FormMessage>
                    </div>
                );
            }}
        />
    );
};

PathInput.displayName = "PathInput";
