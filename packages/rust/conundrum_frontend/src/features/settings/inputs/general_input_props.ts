import { type ReactNode } from "react";
import { type Path, type FieldValues } from "react-hook-form";

export interface GeneralImportProps<FormSchema extends FieldValues> {
    name: Path<FormSchema>;
}

export interface LabeledImportProps<
    ValueType extends FieldValues,
> extends GeneralImportProps<ValueType> {
    label: ReactNode;
    desc?: ReactNode;
}
