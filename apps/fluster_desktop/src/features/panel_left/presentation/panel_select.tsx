import React, { type ReactNode } from "react";
import { PanelLeftItemId, SidePanelItem } from "../state/panel_left_state";
import { PanelRightItemId } from "#/panel_right/state/panel_right_state";
import { Form, GeneralSelectInput } from "@fluster/webview_utils";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

type AnySidePanelId = PanelLeftItemId | PanelRightItemId;

interface PanelSelectProps<T extends AnySidePanelId> {
    onChange: (val: SidePanelItem<T>) => void;
    value: SidePanelItem<T>;
    items: SidePanelItem<T>[];
}

const formSchema = z.object({
    item: z.string(),
});

export const PanelSelect = <T extends AnySidePanelId>(
    props: PanelSelectProps<T>,
): ReactNode => {
    const form = useForm({
        resolver: zodResolver(formSchema),
    });
    form.watch((formState) => {
        const formStateItem = formState.item as T | undefined;
        if (formStateItem && formStateItem !== props.value.id) {
            const foundValue = props.items.find((item) => item.id === formStateItem);
            if (foundValue) {
                props.onChange(foundValue);
            }
        }
    });
    return (
        <Form {...form}>
            <GeneralSelectInput
                label="Panel Select"
                items={props.items.map((item) => {
                    return {
                        label: item.label,
                        value: item.id,
                    };
                })}
                classes={{
                    label: "hidden",
                    formItem: "w-full my-4",
                    selectItem: "w-full",
                    selectTrigger: "w-full",
                }}
                placeholder="Panel"
                form={form}
                name="item"
            />
        </Form>
    );
};

PanelSelect.displayName = "PanelSelect";
