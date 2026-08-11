import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";
import { EmptyFlashcards } from "./empty_flashcards";

export const MainFlashcardsPage = (): ReactNode => {
    return (
        <PageContainer center title="Flashcards">
            <EmptyFlashcards />
        </PageContainer>
    )
};

MainFlashcardsPage.displayName = "MainFlashcardsPage";
