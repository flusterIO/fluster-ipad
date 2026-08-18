import { sortByCtime } from "#/database/shared_queries/sort_queries";
import { rspc } from "@/app/rspc_client";
import { Form } from "@/components/shad/form";
import { paginateSingle } from "@/utils/search_utils";
import { zodResolver } from "@hookform/resolvers/zod";
import React, { type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { aiPrimaryTasks, mcpToolNames } from "@conundrum/ts/codegen-docgen";
import { agentDescriptionSchema } from "@conundrum/ts/schemas";
import { type z } from "zod";
import consola from "consola";

export const AgentForm = ({ agentId }: { agentId: string }): ReactNode => {
    const { data, isLoading } = rspc.useQuery([
        "crud.agent_description.get_by_predicate",
        {
            predicate: agentId ? `agent_id="${agentId}"` : null,
            pagination: paginateSingle,
            sort: [sortByCtime],
        },
    ]);

    const { mutateAsync } = rspc.useMutation(
        "crud.agent_description.update_many",
        {
            onError: (e) => {
                consola.error("Error: ", e);
            },
        },
    );

    const form = useForm({
        resolver: zodResolver(agentDescriptionSchema),
    });

    const handleSubmit = async (
        validData: z.infer<typeof agentDescriptionSchema>,
    ): Promise<void> => {
        const res = await mutateAsync([validData], {
            onError: (e) => {
                consola.error("Error: ", e);
            },
        });
        console.log("res: ", res);
    };
    /* const {} = rspc.useQuery(["crud"]) */
    return (
        <Form {...form}>
            <form
                onSubmit={(e) => {
                    form
                        .handleSubmit(handleSubmit)(e)
                        .catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                }}
            ></form>
        </Form>
    );
};

AgentForm.displayName = "AgentForm";
