import React, { type ReactNode } from "react";
import { type FieldValues, type Path } from "react-hook-form";
import { FormField } from "@/components/shad/form";
import { Checkbox } from "@/components/shad/checkbox";
import { cn } from "@/utils/shad_utils";
import { FormFieldDescOrMessage } from "../form_field_desc_or_message";

interface SimpleLabeledCheckboxProps<T extends FieldValues> {
  label: ReactNode;
  desc?: ReactNode;
  name: Path<T>;
  withBorder?: boolean;
  /**
   * Add a subtle border around the unchecked checkbox
   */
  borderPrimary?: boolean;
  classes?: {
    container?: string;
    checkbox?: string;
    textContainer?: string;
    title?: string;
    desc?: string;
  };
}

export const SimpleLabeledCheckbox = <T extends FieldValues>({
  name,
  label,
  desc,
  withBorder,
  borderPrimary = true,
  classes = {},
}: SimpleLabeledCheckboxProps<T>): ReactNode => {
  return (
    <FormField
      name={name}
      render={() => {
        return (
          <div
            className={cn(
              "w-full grid grid-cols-[auto_1fr] gap-x-2 my-2",
              withBorder && "rounded p-2 border",
              classes.container,
            )}
          >
            <Checkbox
              uncheckedSecondary
              className={cn(
                "place-self-center",
                borderPrimary && "border-primary/50!",
                classes.checkbox,
              )}
            />
            <div
              className={cn(
                "flex flex-col justify-center items-start",
                classes.textContainer,
              )}
            >
              <div className={cn("", classes.title)}>{label}</div>
              <FormFieldDescOrMessage desc={desc} />
            </div>
          </div>
        );
      }}
    />
  );
};

SimpleLabeledCheckbox.displayName = "SimpleLabeledCheckbox";
