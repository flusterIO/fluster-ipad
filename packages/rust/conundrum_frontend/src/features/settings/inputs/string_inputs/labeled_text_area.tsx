import { FormMessage } from "@/components/shad/form";
import { Label } from "@/components/shad/label";
import { Textarea } from "@/components/shad/textarea";
import { cn } from "@/utils/shad_utils";
import { capitalize } from "@/utils/string_utilts";
import React, { type ComponentProps, type ReactNode } from "react";
import { type FieldValues, type Path, useFormContext } from "react-hook-form";

interface LabeledTextAreaInputProps<T extends FieldValues> {
    label?: ReactNode;
    name: Path<T>;
    desc?: string;
    classes?: {
        container?: string;
        input?: string;
    };
    taProps?: Omit<
        ComponentProps<typeof Textarea>,
        "value" | "onChange" | "className"
    >;
}

export const LabeledTextAreaInput = <T extends FieldValues>({
    label,
    name,
    desc,
    taProps,
    classes = {},
}: LabeledTextAreaInputProps<T>): ReactNode => {
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
            <Textarea
                {...taProps}
                value={val ?? ""}
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

LabeledTextAreaInput.displayName = "LabeledTextAreaInput";
