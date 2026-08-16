import React, { type ReactNode } from "react";
import { AISettingSection } from "./settings_sections/ai_settings/ai_settings_section";
import { NotificationSettings } from "./settings_sections/notifications/notification_settings";
import { Form } from "@/components/shad/form";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { mainSettingsSchema } from "./settings_schema/main_settings_schema";
import { type z } from "zod";

export const MainSettingsPage = (): ReactNode => {
    const form = useForm({
        resolver: zodResolver(mainSettingsSchema),
        defaultValues: {
            name: {
                first: "",
                middle: "",
                last: "",
            },
        },
    });

    const handleSubmit = (
        data: z.infer<typeof mainSettingsSchema>,
    ): Promise<void> => {
        console.log("data: ", data);
    };
    return (
        <Form {...form}>
            <form
                className="@container/settings w-full max-w-270 mx-auto px-6"
                onSubmit={form.handleSubmit(handleSubmit)}
            >
                <AISettingSection />
                <NotificationSettings />
            </form>
        </Form>
    );
};

MainSettingsPage.displayName = "MainSettingsPage";
