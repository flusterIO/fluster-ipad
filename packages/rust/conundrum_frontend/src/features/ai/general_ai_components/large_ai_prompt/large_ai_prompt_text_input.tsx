import { Button } from '@/components/shad/button';
import { FormMessage } from '@/components/shad/form';
import { Textarea } from '@/components/shad/textarea';
import { cn } from '@/utils/shad_utils';
import React, { type ComponentProps, type ReactNode } from 'react'
import { type FieldValues, type Path, useFormContext } from 'react-hook-form'


export type LargeAIPromptViewMode = "horizontal" | "vertical";

interface LargeAIPromptTextInputProps<T extends FieldValues> {
    inputProps?: Omit<ComponentProps<typeof Textarea>, "value" | "onChange">;
    name: Path<T>
    viewMode: LargeAIPromptViewMode
    classes?: {
        container?: string
        input?: string
    }
    label: ReactNode
    desc?: ReactNode
}

export const LargeAIPromptTextInput = <T extends FieldValues>({ name, label, inputProps, desc, viewMode, classes = {} }: LargeAIPromptTextInputProps<T>): ReactNode => {
    const form = useFormContext<T>();
    const value = form.watch(name);
    return (
        <div
            className={cn("overflow-y-auto overflow-x-hidden", {
                horizontal: "w-full h-full flex flex-col justify-center items-center",
                vertical: "w-full h-fit min-h-62.5 flex-col justify-center items-center"
            }[viewMode], classes.container)}
        >
            <div className="w-full h-fit flex flex-col justify-center items-start gap-y-2 px-3">
                <h3 className="text-xl font-bold text-foreground">{label}</h3>
                <Textarea
                    {...inputProps}
                    value={value}
                    onChange={(e) => {
                        form.setValue(name, e.target.value as T[typeof name])
                    }}
                    className="border-primary! text-lg font-semibold focus-visible:border-primary/40!"
                />
                <FormMessage>
                    {desc ? (
                        <div className="text-sm text-foreground/80!">{desc}</div>
                    ) : null}
                </FormMessage>
                <div className="w-full h-fit flex flex-row justify-end items-center">
                    <Button type="submit">
                        Generate
                    </Button>
                </div>
            </div>
        </div>
    )
}


LargeAIPromptTextInput.displayName = "LargeAIPromptTextInput"
