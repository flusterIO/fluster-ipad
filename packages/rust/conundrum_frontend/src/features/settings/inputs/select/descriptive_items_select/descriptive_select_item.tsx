import React, { type ReactNode } from "react";
import { type DescriptiveItem } from "./descriptive_items_select";
import { useFormContext, type FieldValues, type Path } from "react-hook-form";
import { Checkbox } from "@/components/shad/checkbox";

interface DescriptiveSelectItemProps<T extends FieldValues> {
    item: DescriptiveItem;
    formName: Path<T>;
}

export const DescriptiveSelectItem = <T extends FieldValues>({
    item,
    formName,
}: DescriptiveSelectItemProps<T>): ReactNode => {
    const form = useFormContext<T>();
    const value = form.getValues(formName);
    const isActive = value === item.value;
    return (
        <div className="w-full h-fit rounded border bg-fd-card px-3 py-2 grid grid-cols-[auto_1fr] gap-x-3">
            <div className="flex flex-col justify-center items-center">
                <Checkbox
                    uncheckedSecondary
                    borderPrimary
                    checked={isActive}
                    onCheckedChange={(v) => {
                        if (v) {
                            form.setValue(formName, item.value as T[typeof formName]);
                        }
                    }}
                />
            </div>
            <div className="flex flex-col justify-center items-start">
                <div className="font-semibold text-foreground">{item.label}</div>
                <div className="text-foreground/60 text-sm">{item.desc}</div>
            </div>
        </div>
    );
};

DescriptiveSelectItem.displayName = "DescriptiveSelectItem";
