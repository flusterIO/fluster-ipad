import React, { useState, type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";
import { RecentNoteDashboardListItem } from "./recent_note_dashboard_list_item/recent_note_dashboard_list_item";
import { useRecentNotes } from "./use_recent_notes";
import { Button } from "@/components/shad/button";
import { ChevronLeftIcon, ChevronRightIcon } from "lucide-react";
import { cn } from "@/utils/shad_utils";

export const RecentNotesCard = (): ReactNode => {
    const [page, setPage] = useState(1);
    const [perPage, setPerPage] = useState(10);
    const [notes, isLoading] = useRecentNotes({
        page,
        per_page: perPage,
    });

    return (
        <ModularDashboardCard
            title="Recent Notes"
            options={{
                "Per Page": [10, 20, 50, 100].map((n) => {
                    return {
                        label: `${n}`,
                        action: () => {
                            setPerPage(n);
                        },
                    };
                }),
            }}
            footer={
                <div className="w-full h-fit flex flex-row justify-end items-center">
                    <Button
                        onClick={() => {
                            setPage(page - 1);
                        }}
                        disabled={page <= 1}
                        size={"icon-sm"}
                        variant={"ghost"}
                    >
                        <ChevronLeftIcon />
                    </Button>
                    <Button
                        onClick={() => {
                            setPage(page + 1);
                        }}
                        size={"icon-sm"}
                        variant={"ghost"}
                    >
                        <ChevronRightIcon />
                    </Button>
                </div>
            }
        >
            <div
                className={cn(
                    "grow",
                    notes?.length ? "flex-col justify-start items-start" : "",
                )}
            >
                {notes?.length ? (
                    notes.map((n) => {
                        return <RecentNoteDashboardListItem item={n} key={n.id} />;
                    })
                ) : (
                    <EmptyCardDataText>
                        {isLoading ? "Loading" : "No notes to show"}
                    </EmptyCardDataText>
                )}
            </div>
        </ModularDashboardCard>
    );
};

RecentNotesCard.displayName = "RecentNotesCard";
