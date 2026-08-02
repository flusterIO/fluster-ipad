import { type ReactNode } from "react";
import {
    type Path,
    type FieldValues,
    type UseFormReturn,
} from "react-hook-form";

export interface GeneralImportProps<FormSchema extends FieldValues> {
    name: Path<FormSchema>;
    form: UseFormReturn<FormSchema>;
}

export interface LabeledImportProps<
    ValueType extends FieldValues,
> extends GeneralImportProps<ValueType> {
    label: ReactNode;
    desc?: ReactNode;
}
