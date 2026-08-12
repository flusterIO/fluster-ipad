import { FormMessage } from "@/components/shad/form";
import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import { cn } from "@/utils/shad_utils";
import { capitalize } from "@/utils/string_utilts";
import React, { type ReactNode } from "react";
import { type FieldValues, type Path, useFormContext } from "react-hook-form";

interface LabeledStringInputProps<T extends FieldValues> {
    label?: ReactNode;
    name: Path<T>;
    desc?: string;
    classes?: {
        container?: string;
        input?: string;
    };
}

export const LabeledStringInput = <T extends FieldValues>({
    label,
    name,
    desc,
    classes = {},
}: LabeledStringInputProps<T>): ReactNode => {
    const form = useFormContext<T>();
    const val = form.watch(name);
    return (
        <div
            className={cn(
                "w-full flex flex-col justify-center items-start gap-y-2",
                classes.container,
            )}
        >
            <Label>{label ?? capitalize(name)}</Label>
            <Input
                value={val}
                onChange={(e) => {
                    form.setValue(name, e.target.value as T[typeof name]);
                }}
                className={classes.input}
            />
            <FormMessage>
                {desc ? (
                    <div className="text-sm text-foreground/80!">{desc}</div>
                ) : null}
            </FormMessage>
        </div>
    );
};

LabeledStringInput.displayName = "LabeledStringInput";
