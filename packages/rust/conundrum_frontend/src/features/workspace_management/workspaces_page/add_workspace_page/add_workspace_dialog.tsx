import { PathInput } from "#/settings/inputs/path_input/path_input";
import { ComposedDialog } from "@/components/shad/dialog";
import React, { type ReactNode } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Form } from "@/components/shad/form";
import { Button } from "@/components/shad/button";
import { rspc } from "@/app/rspc_client";
import { SimpleLabeledCheckbox } from "#/settings/inputs/boolean_inputs/simple_labeled_checkbox";
import { InlineCode } from "#/ui/typography/inline_code";
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
    respect_gitignore: z.boolean(),
    ignore_hidden: z.boolean(),
    label: z.string(),
  });
  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      path: "",
      label: "",
      respect_gitignore: true,
      ignore_hidden: true,
    },
  });
  const addWorkspace = (
    path: string,
    label: string,
    respect_gitignore: boolean,
    ignore_hidden: boolean,
  ): void => {
    const res = rspc.useQuery(
      [
        "user_workspace_crud.save_many",
        [
          {
            root: path,
            respect_gitignore,
            ignore_hidden,
            label: label === "" ? null : label,
          },
        ],
      ],
      {},
    );
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
        <div className="@container/modalContent grid grid-cols-1 gap-y-4">
          <PathInput form={form} name={"path"} label={"Path"} />
          <div className="w-full flex flex-col justify-start items-start @[640px]/modalContent:grid @[640px]/modalContent:grid-cols-2">
            <SimpleLabeledCheckbox<typeof form>
              name={"respect_gitignore"}
              desc={
                <>
                  Ignore files using any <InlineCode code=".gitignore" /> files
                  found.
                </>
              }
              label={
                <>
                  Respect <InlineCode code=".gitignore" />
                </>
              }
              /* classes={{ container: "my-0" }} */
            />
            <SimpleLabeledCheckbox<typeof form>
              name="ignore_hidden"
              label="Ignore Hidden"
              desc="Ignore files that are hidden by your file system."
              /* classes={{ container: "my-0" }} */
            />
          </div>
          <div className="w-full flex flex-row justify-end items-center">
            <Button
              onClick={() => {
                const { path, label, respect_gitignore, ignore_hidden } =
                  form.getValues();
                addWorkspace(path, label, respect_gitignore, ignore_hidden);
              }}
            >
              Create
            </Button>
          </div>
        </div>
      </Form>
    </ComposedDialog>
  );
};

AddWorkspaceDialog.displayName = "AddWorkspaceDialog";
