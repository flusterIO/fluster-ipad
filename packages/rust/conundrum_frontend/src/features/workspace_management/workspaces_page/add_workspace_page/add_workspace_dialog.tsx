import { PathInput } from "#/settings/inputs/path_input/path_input";
import { ComposedDialog } from "@/components/shad/dialog";
import React, { type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Form } from "@/components/shad/form";

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
                <PathInput form={form} name={"path"} label={"Path"} />
            </Form>
        </ComposedDialog>
    );
};

AddWorkspaceDialog.displayName = "AddWorkspaceDialog";
