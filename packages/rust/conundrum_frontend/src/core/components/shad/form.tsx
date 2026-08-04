"use client";

import * as React from "react";
import type * as LabelPrimitive from "@radix-ui/react-label";
import { Slot } from "@radix-ui/react-slot";
import {
    motion,
    AnimatePresence,
    MotionProps,
    HTMLMotionProps,
} from "framer-motion";
import {
    Controller,
    FormProvider,
    useFormContext,
    useFormState,
    type ControllerProps,
    type FieldPath,
    type FieldValues,
} from "react-hook-form";
import { cn } from "@/utils/shad_utils";
import { Label } from "./label";

const Form = FormProvider;

interface FormFieldContextValue<
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> {
    name: TName;
}

const FormFieldContext = React.createContext<FormFieldContextValue>(
    {} as FormFieldContextValue,
);

const FormField = <
    TFieldValues extends FieldValues = FieldValues,
    TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
    ...props
}: ControllerProps<TFieldValues, TName>) => {
    return (
        <FormFieldContext.Provider value={{ name: props.name }}>
            <Controller {...props} />
        </FormFieldContext.Provider>
    );
};

const useFormField = () => {
    const fieldContext = React.useContext(FormFieldContext);
    const itemContext = React.useContext(FormItemContext);
    const { getFieldState } = useFormContext();
    const formState = useFormState({ name: fieldContext.name });
    const fieldState = getFieldState(fieldContext.name, formState);

    if (!fieldContext) {
        throw new Error("useFormField should be used within <FormField>");
    }

    const { id } = itemContext;

    return {
        id,
        name: fieldContext.name,
        formItemId: `${id}-form-item`,
        formDescriptionId: `${id}-form-item-description`,
        formMessageId: `${id}-form-item-message`,
        ...fieldState,
    };
};

interface FormItemContextValue {
    id: string;
}

const FormItemContext = React.createContext<FormItemContextValue>(
    {} as FormItemContextValue,
);

function FormItem({ className, ...props }: React.ComponentProps<"div">) {
    const id = React.useId();

    return (
        <FormItemContext.Provider value={{ id }}>
            <div
                data-slot="form-item"
                className={cn("grid gap-2", className)}
                {...props}
            />
        </FormItemContext.Provider>
    );
}

function FormLabel({
    className,
    ...props
}: React.ComponentProps<typeof LabelPrimitive.Root>) {
    const { error, formItemId } = useFormField();

    return (
        <Label
            data-slot="form-label"
            data-error={!!error}
            className={cn("data-[error=true]:text-destructive", className)}
            htmlFor={formItemId}
            {...props}
        />
    );
}

function FormControl({ ...props }: React.ComponentProps<typeof Slot>) {
    const { error, formItemId, formDescriptionId, formMessageId } =
        useFormField();

    return (
        <Slot
            data-slot="form-control"
            id={formItemId}
            aria-describedby={
                !error ? formDescriptionId : `${formDescriptionId} ${formMessageId}`
            }
            aria-invalid={!!error}
            {...props}
        />
    );
}

function FormDescription({ className, ...props }: React.ComponentProps<"p">) {
    const { formDescriptionId } = useFormField();

    return (
        <p
            data-slot="form-description"
            id={formDescriptionId}
            className={cn("text-muted-foreground text-sm", className)}
            {...props}
        />
    );
}

function FormMessageInner({
    className,
    children = null,
    ...props
}: HTMLMotionProps<"p">) {
    const { error, formMessageId } = useFormField();
    const body = error ? (error?.message ?? "") : children;

    if (children) {
        return (
            <motion.div
                className={cn("relative h-fit w-full ", className)}
                animate={body ? "error" : "desc"}
                initial={body ? "error" : "desc"}
            >
                <motion.div
                    className="relative text-sm text-foreground/80!"
                    variants={{
                        error: {
                            y: -100,
                            opacity: 0,
                        },
                        desc: {
                            y: 0,
                            opacity: 1,
                        },
                    }}
                >
                    {children}
                </motion.div>
                <motion.div
                    className="absolute text-sm h-full max-h-full left-0 right-0 bottom-0 top-0 text-destructive!"
                    variants={{
                        error: {
                            opacity: 1,
                            y: 0,
                        },
                        desc: {
                            opacity: 0,
                            y: 100,
                        },
                    }}
                >
                    {body ?? null}
                </motion.div>
            </motion.div>
        );
    }

    return (
        <motion.p
            data-slot="form-message"
            id={formMessageId}
            key={formMessageId}
            className={cn("text-destructive! text-sm", className)}
            {...props}
            initial={{
                height: 0,
            }}
            animate={{
                height: "auto",
            }}
            exit={{
                height: 0,
            }}
        >
            {body}
        </motion.p>
    );
}

const FormMessage = (
    props: React.ComponentProps<typeof FormMessageInner>,
): React.ReactNode => {
    return (
        <AnimatePresence>
            <FormMessageInner {...props} />
        </AnimatePresence>
    );
};

export {
    useFormField,
    Form,
    FormItem,
    FormLabel,
    FormControl,
    FormDescription,
    FormMessage,
    FormField,
};
