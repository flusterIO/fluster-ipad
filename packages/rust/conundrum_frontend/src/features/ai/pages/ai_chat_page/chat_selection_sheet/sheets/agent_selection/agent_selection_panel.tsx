import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import {
    InputGroup,
    InputGroupInput,
    InputGroupAddon,
} from "@/components/shad/input-group";
import { SheetHeader, SheetTitle } from "@/components/shad/sheet";
import { SearchIcon } from "lucide-react";
import React, { type ReactNode } from "react";
import { AgentSelectionItem } from "./agent_selection_item";
import { rspc } from "@/app/rspc_client";
import { useSearchParams } from "react-router";

export const AgentSelectionPanel = (): ReactNode => {
    const { isLoading, data: agents } = rspc.useQuery([
        "crud.agent_description.get_by_predicate",
        {
            predicate: null,
            pagination: {
                page: 1,
                per_page: 10,
            },
        },
    ]);
    const [sp] = useSearchParams();
    const agent = sp.get("agent");
    return (
        <>
            <SheetHeader className="w-full">
                <SheetTitle className="w-full">Chat History</SheetTitle>
            </SheetHeader>
            <div className="px-4 grow w-full">
                <InputGroup className="px-4 text-sm focus-visible:ring-0! focus-visible:border-none">
                    <InputGroupInput className="p-0" />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                </InputGroup>
                {isLoading ? (
                    <CenteredExpandedLoadingIndicator className="grow" />
                ) : agents?.length ? (
                    agents.map((c) => {
                        return (
                            <AgentSelectionItem
                                active={agent === c.id}
                                agent={c}
                                key={c.id}
                            />
                        );
                    })
                ) : (
                    <div className="grow w-full h-full flex flex-col justify-center items-center">
                        <h6 className="text-center text-lg font-semibold text-foreground">
                            No History
                        </h6>
                        <div className="text-center text-foreground/80">
                            You don't have any chat history to display. Simply start a
                            conversation and AI will take care of the rest
                        </div>
                    </div>
                )}
            </div>
        </>
    );
};

AgentSelectionPanel.displayName = "AgentSelectionPanel";
