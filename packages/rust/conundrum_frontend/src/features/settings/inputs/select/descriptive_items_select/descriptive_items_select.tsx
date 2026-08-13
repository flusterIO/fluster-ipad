import React, { type ReactNode } from "react";
import { DescriptiveSelectItem } from "./descriptive_select_item";
import { type FieldValues, type Path } from "react-hook-form";
import { cn } from "@/utils/shad_utils";

export interface DescriptiveItem {
    label: ReactNode;
    desc: ReactNode;
    value: string;
    id: string;
}

export interface DescriptiveItemsSelectProps<T extends FieldValues> {
    options: DescriptiveItem[];
    name: Path<T>;
    label?: ReactNode;
    desc?: ReactNode;
}

export const DescriptiveItemsSelect = <T extends FieldValues>({
    options,
    name,
    label,
    desc,
}: DescriptiveItemsSelectProps<T>): ReactNode => {
    return (
        <div className="w-full h-fit">
            {label || desc ? (
                <>
                    <h3
                        className={cn("text-xl text-foreground font-bold", !desc && "mb-4")}
                    >
                        {label}
                    </h3>
                    {desc ? <div className="text-foreground mb-4">{desc}</div> : null}
                </>
            ) : null}
            <div className="w-full h-fit flex flex-col justify-center items-center gap-y-4">
                {options.map((item) => {
                    return (
                        <DescriptiveSelectItem formName={name} item={item} key={item.id} />
                    );
                })}
            </div>
        </div>
    );
};

DescriptiveItemsSelect.displayName = "DescriptiveItemsSelect";
