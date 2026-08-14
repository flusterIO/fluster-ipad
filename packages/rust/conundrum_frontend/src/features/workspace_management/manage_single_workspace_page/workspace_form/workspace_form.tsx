import {
    type WorkspaceUpdateRequest,
    type WorkspaceByPredicate,
} from "#/database/db_utility_types/workspace";
import { useGenericRemoteDataContext } from "#/database/state/generic_data_loading_context/generic_data_loading_context";
import { zodResolver } from "@hookform/resolvers/zod";
import React, { useEffect, type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { userWorkspaceSchema } from "@conundrum/ts/schemas";
import { Form } from "@/components/shad/form";
import { PathInput } from "#/settings/inputs/path_input/path_input";
import { LabeledStringInput } from "#/settings/inputs/string_inputs/labeled_string_input";
import { SimpleLabeledCheckbox } from "#/settings/inputs/boolean_inputs/simple_labeled_checkbox";
import { PlainInlineCode } from "#/ui/typography/inline_code";
import { AINotepadSettings } from "#/settings/model_setting_sections/ai_notepad_settings";
import { Button } from "@/components/shad/button";
import { defaultAINotepadSchema } from "@conundrum/ts/schemas";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { useLogger } from "#/logging/state/hooks/use_logger";

export const WorkspaceForm = (): ReactNode => {
    const { data } = useGenericRemoteDataContext<{
        workspace?: WorkspaceByPredicate;
    }>();
    const { mutateAsync } = rspc.useMutation("crud.user_workspace.update_many");
    const logger = useLogger();
    const form = useForm<WorkspaceUpdateRequest[number]>({
        resolver: zodResolver(userWorkspaceSchema),
        defaultValues: (data?.workspace
            ? {
                ...data.workspace,
                resource_dir: data.workspace.resource_dir ?? null,
            }
            : {
                resource_dir: "",
                label: "",
                ignore_hidden: false,
                respect_gitignore: false,
                root: "",
                ...defaultAINotepadSchema,
            }) satisfies WorkspaceUpdateRequest[number],
    });

    useEffect(() => {
        if (data?.workspace) {
            const ws = data.workspace;
            form.setValue("resource_dir", ws.resource_dir ?? null);
            form.setValue("label", ws.label);
            form.setValue("ignore_hidden", ws.ignore_hidden);
            form.setValue("respect_gitignore", ws.respect_gitignore);
            form.setValue("root", ws.root);
            form.setValue("ai.notes", ws.ai.notes);
            form.setValue("ai.ai_generated_input", ws.ai.ai_generated_input);
        }
    }, [data]);
    const handleSubmit = async ({
        root,
        ...data
    }: WorkspaceUpdateRequest[number]) => {
        if (!root) {
            return undefined;
        }
        const workspaceUpdate: WorkspaceUpdateRequest = [
            {
                root,
                ai: data.ai ?? {
                    ai_generated_input: "",
                    notes: "",
                },
                ignore_hidden: data.ignore_hidden,
                respect_gitignore: data.respect_gitignore,
                label: data.label ?? null,
                resource_dir: data.resource_dir ?? null,
            },
        ];
        try {
            await mutateAsync(workspaceUpdate);
            await logger({
                title: "Success",
                severity: "success",
                message: "Workspace updated successfully",
                purpose: "entity-updated",
                ai_description: `The user just updated their workspace at \`${root}\`.`,
            });
        } catch (err: unknown) {
            consola.error("Error: ", err);
        }
    };
    return (
        <div className="@container/form">
            <Form {...form}>
                <form
                    className="my-6 space-y-6"
                    onSubmit={(e) => {
                        form
                            .handleSubmit(handleSubmit)(e)
                            .catch((err: unknown) => {
                                console.error("Error: ", err);
                            });
                    }}
                >
                    <h2 className="text-3xl font-bold text-foreground">
                        Workspace Settings
                    </h2>
                    <PathInput
                        name="root"
                        label="File System Path"
                        desc="This is the path to the root of your workspace. This workspace can be as deeply nested as you like, but nesting workspaces within one another may cause unexpected behavior."
                        classes={{
                            input: "w-full",
                            container: "w-full",
                        }}
                        disabled
                    />
                    <LabeledStringInput
                        name="label"
                        desc="For your own reference, but AI will see this as well."
                    />
                    <h3 className="text-2xl font-semibold text-foreground">
                        Search Behavior
                    </h3>
                    <SimpleLabeledCheckbox
                        name="ignore_hidden"
                        label={"Ignore Hidden"}
                        desc="File based search should ignore files if they are hidden by your operating system. These types of files are common for developers, but others should probably set this to true."
                    />
                    <SimpleLabeledCheckbox
                        name="respect_gitignore"
                        label={
                            <>
                                Respect <PlainInlineCode code=".gitignore" />
                            </>
                        }
                        desc={
                            <>
                                Respect any <PlainInlineCode code=".gitignore" /> files found,
                                ignoring files matching the patterns discovered. You can also
                                use a <PlainInlineCode code=".cdrm_ignore" /> file if you want
                                to keep files in git, but ignore them for Conundrum.
                            </>
                        }
                    />
                    <AINotepadSettings />
                    <div className="w-full flex flex-row justify-end items-center">
                        <Button type="submit">Save</Button>
                    </div>
                </form>
            </Form>
        </div>
    );
};

WorkspaceForm.displayName = "WorkspaceForm";
