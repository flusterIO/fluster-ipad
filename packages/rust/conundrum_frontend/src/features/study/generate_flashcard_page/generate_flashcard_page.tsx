import { LargeAIPrompt } from "#/ai/general_ai_components/large_ai_prompt/large_ai_prompt";
import { generalPromptSchema } from "#/ai/schemas/general_prompt_schema";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { PageContainer } from "@/components/general/page_container";
import { Button } from "@/components/shad/button";
import { Form } from "@/components/shad/form";
import { zodResolver } from "@hookform/resolvers/zod";
import React, { type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { type z } from "zod";

export const GenerateFlashcardPage = (): ReactNode => {
    const form = useForm({
        resolver: zodResolver(generalPromptSchema),
        defaultValues: {
            prompt: "",
        },
    });
    const onSubmit = async (data: z.infer<typeof generalPromptSchema>): Promise<void> => {
        console.log("data: ", data);
    }
    return (
        <PageContainer
            itemClasses="min-h-[calc(100vh-4rem)]"
        >
            <Form {...form}>
                <form className="w-full h-full" onSubmit={(data) => {
                    form.handleSubmit(onSubmit)(data).catch((err: unknown) => {
                        logMaybeObject("Error: ", err)
                    })
                }}>
                    <LargeAIPrompt promptDescription={<>Prompt AI for flashcards and it should be able to query data throughout your database to understand what you're already aware of, what you're studying, and what you're working on.</>} name="prompt">Here</LargeAIPrompt>
                </form>
            </Form>
        </PageContainer>
    );
};

GenerateFlashcardPage.displayName = "GenerateFlashcardPage";
