import React, { useEffectEvent, type ReactNode } from "react";
import {
    LabeledStringInput,
    type LabeledStringInputProps,
} from "../inputs/string_inputs/labeled_string_input";
import { useForm, type FieldValues } from "react-hook-form";
import { type UniqueSettingKey } from "#/database/db_utility_types/settings";
import { Form } from "@/components/shad/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { rspc } from "@/app/rspc_client";
import { type Procedures } from "@/codegen/bindings";
import { useEventCallback } from "@/state/hooks/use_event_callback";
import consola from "consola";

interface LabeledStringSettingProps<
    T extends FieldValues,
    K extends UniqueSettingKey,
> extends LabeledStringInputProps<T> {
    settingKey: K;
    formatter?: (val: Procedures["settings"]["read"]["output"]) => string;
}

const schema = z.object({
    value: z.string(),
});

export const LabeledStringSetting = <
    T extends FieldValues,
    K extends UniqueSettingKey,
>({
    settingKey,
    ...props
}: LabeledStringSettingProps<T, K>): ReactNode => {
    const value = rspc.useQuery(["settings.read", settingKey]);
    const { mutateAsync } = rspc.useMutation("settings.save");
    const form = useForm({
        resolver: zodResolver(schema),
        defaultValues: {
            value: "",
        },
    });
    const updateSettings = useEffectEvent(() => {
        const data = form.getValues();
        mutateAsync({});
    });
    return (
        <Form {...form}>
            <LabeledStringInput {...props} />
        </Form>
    );
};

LabeledStringSetting.displayName = "LabeledStringSetting";
