import { PathInput } from "#/settings/inputs/path_input/path_input";
import { ComposedDialog } from "@/components/shad/dialog";
import React, { type ReactNode, useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import consola from "consola";
import { Form } from "@/components/shad/form";
import { Button } from "@/components/shad/button";
import { rspc } from "@/app/rspc_client";
import { SimpleLabeledCheckbox } from "#/settings/inputs/boolean_inputs/simple_labeled_checkbox";
import { InlineCode, PlainInlineCode } from "#/ui/typography/inline_code";
import { Procedures } from "@/codegen/bindings";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { useLogger } from "#/logging/state/hooks/use_logger";

export const AddWorkspaceDialog = ({
    open,
    close,
}: {
    open: boolean;
    close: () => void;
}): ReactNode => {
    const logger = useLogger();
    const formSchema = z.object({
        path: z.string(),
        respect_gitignore: z.boolean(),
        ignore_hidden: z.boolean(),
        label: z.string(),
        resourceDir: z.string().optional(),
        ai: z.object({
            notes: z.string(),
            ai_generated_input: z.string(),
        }),
    });
    const form = useForm({
        resolver: zodResolver(formSchema),
        defaultValues: {
            path: "",
            label: "",
            respect_gitignore: true,
            ignore_hidden: true,
            ai: {
                notes: "",
                ai_generated_input: "",
            },
        },
    });
    const [pathExists, setPathExists] = useState(true);
    const { mutateAsync } = rspc.useMutation("crud.user_workspace.save_many", {
        onError: (err) => {
            logMaybeObject(
                "An error occurred while attempting to save this entry: ",
                err,
            );
        },
    });

    const addWorkspace = async (
        data: Procedures["user_workspace_crud"]["save_many"]["input"][0],
    ): Promise<void> => {
        try {
            await mutateAsync([data]);
            logger({
                title: "Success",
                message: "This workspace has been created successfully.",
                ai_description: `The user has created a new workspace at \`${data.root}\`.`,
                severity: "Success",
                purpose: "entity-created",
            });
            window.dispatchEvent(
                new CustomEvent("workspace-add", {
                    detail: undefined,
                }),
            );
        } catch (err: unknown) {
            console.log("Error: ", err);
        }
    };

    const onSubmit = async ({
        path,
        label,
        respect_gitignore,
        ignore_hidden,
        ai,
    }: typeof formSchema): void => {
        await addWorkspace({
            root: path,
            label: label.trim() === "" ? null : label,
            respect_gitignore,
            ignore_hidden,
            bib_paths: [],
            resource_dir: undefined,
            ai: ai,
        });
        form.reset();
        close();
    };

    return (
        <ComposedDialog
            title="Add a workspace"
            dialogProps={{
                open,
                onOpenChange: (b) => {
                    if (!b) {
                        close();
                    }
                },
            }}
            classes={{
                content: "max-w-[min(768px,90vw)] min-w-[min(768px,90vw)]",
            }}
        >
            <Form {...form}>
                <form
                    className="@container/modalContent grid grid-cols-1 gap-y-4"
                    onSubmit={form.handleSubmit(onSubmit, (err) => {
                        consola.error(`An error occurred while creating this workspace.`);
                    })}
                >
                    <PathInput
                        form={form}
                        name={"path"}
                        label={"Path"}
                        onPathExistsChange={setPathExists}
                        desc="This path will be unchangable until the framework is further developed. While Conundrum is in pre-release, moving a workspace will require creating a new workspace."
                    />
                    <div className="w-full flex flex-col justify-start items-start @[640px]/modalContent:grid @[640px]/modalContent:grid-cols-2 @[640px]/modalContent:gap-x-4">
                        <SimpleLabeledCheckbox<typeof form>
                            name={"respect_gitignore"}
                            desc={
                                <>
                                    File system searches will ignore any matches found in any{" "}
                                    <PlainInlineCode code=".gitignore" /> files in that directory.
                                </>
                            }
                            label={
                                <>
                                    Respect <PlainInlineCode code=".gitignore" />
                                </>
                            }
                        />
                        <SimpleLabeledCheckbox<typeof form>
                            name="ignore_hidden"
                            label="Ignore Hidden"
                            desc="Ignore files that are hidden by your file system."
                        />
                    </div>
                    <div className="w-full flex flex-row justify-end items-center">
                        <Button type="submit">Create</Button>
                    </div>
                </form>
            </Form>
        </ComposedDialog>
    );
};

AddWorkspaceDialog.displayName = "AddWorkspaceDialog";
