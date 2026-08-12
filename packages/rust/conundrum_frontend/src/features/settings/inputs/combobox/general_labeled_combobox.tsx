import React, { type ReactNode } from "react";
import { GeneralCombobox, type GeneralComboboxProps } from "./general_combobox";
import { Label } from "@/components/shad/label";
import { cn } from "@/utils/shad_utils";

interface GeneralLabeledComboboxProps extends GeneralComboboxProps {
    label: ReactNode;
    className?: string;
}

export const GeneralLabeledCombobox = ({
    label,
    className,
    ...props
}: GeneralLabeledComboboxProps): ReactNode => {
    return (
        <div
            className={cn(
                "w-fit flex flex-col justify-center items-start gap-y-2",
                className,
            )}
        >
            <Label>{label}</Label>
            <GeneralCombobox {...props} />
        </div>
    );
};

GeneralLabeledCombobox.displayName = "GeneralLabeledCombobox";
