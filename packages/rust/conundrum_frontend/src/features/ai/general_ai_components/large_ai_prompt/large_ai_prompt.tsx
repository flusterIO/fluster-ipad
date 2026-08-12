import { type Textarea } from "@/components/shad/textarea";
import { cn } from "@/utils/shad_utils";
import { AnimatePresence } from "framer-motion";
import React, { type ReactNode, type ComponentProps } from "react";
import { type FieldValues, type Path } from "react-hook-form";
import { useMediaQuery } from "react-responsive"
import { LargeAIPromptTextInput } from "./large_ai_prompt_text_input";
import { LargeAIPromptOutput } from "./large_ai_prompt_output";

interface LargeAIPromptProps<T extends FieldValues> {
    inputProps?: Omit<ComponentProps<typeof Textarea>, "value" | "onChange">;
    children: ReactNode;
    className?: string;
    name: Path<T>;
    promptDescription?: ReactNode
}

export const LargeAIPrompt = <T extends FieldValues>({
    inputProps,
    children,
    className,
    name,
    promptDescription
}: LargeAIPromptProps<T>): ReactNode => {
    const isVertical = useMediaQuery({
        maxWidth: 768
    })
    const label = "Generate Flashcards"
    return (
        <div className={cn({
            vertical: "flex flex-col justify-center items-center h-full",
            horizontal: "grid grid-cols-2 gap-x-4 h-full min-h-[calc(100vh-4rem)] place-items-center",
        }[isVertical ? "vertical" : "horizontal"], className)}>
            <AnimatePresence key={isVertical ? "vertical" : "horizontal"}>
                {isVertical ? (
                    <>
                        <LargeAIPromptTextInput desc={promptDescription} label={label} inputProps={inputProps} name={name} viewMode="vertical" />
                        <LargeAIPromptOutput content={children} viewMode="vertical" />
                    </>
                ) : (
                    <>

                        <LargeAIPromptTextInput desc={promptDescription} label={label} inputProps={inputProps} name={name} viewMode="horizontal" />
                        <LargeAIPromptOutput content={children} viewMode="vertical" />
                    </>
                )}
            </AnimatePresence>
        </div>
    );
};

LargeAIPrompt.displayName = "LargeAIPrompt";
