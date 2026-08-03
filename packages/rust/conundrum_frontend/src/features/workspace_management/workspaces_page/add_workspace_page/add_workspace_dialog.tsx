import { PathInput } from "#/settings/inputs/path_input/path_input";
import { ComposedDialog } from "@/components/shad/dialog";
import React, { type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Form } from "@/components/shad/form";
import { Button } from "@/components/shad/button";
/* import {} from "@/codegen/bindings" */

export const AddWorkspaceDialog = ({
    open,
    close,
}: {
    open: boolean;
    close: () => void;
}): ReactNode => {
    const formSchema = z.object({
        path: z.string(),
    });
    const form = useForm({
        resolver: zodResolver(formSchema),
        defaultValues: {
            path: "",
        },
    });
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
        >
            <Form {...form}>
                <PathInput
                    className="text-sm"
                    form={form}
                    name={"path"}
                    label={"Path"}
                />
                <div className="w-full flex flex-row justify-end items-center mt-4">
                    <Button>Create</Button>
                </div>
            </Form>
        </ComposedDialog>
    );
};

AddWorkspaceDialog.displayName = "AddWorkspaceDialog";
