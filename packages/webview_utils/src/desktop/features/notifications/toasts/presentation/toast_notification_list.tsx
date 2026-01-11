import React, { useState, type ReactNode } from "react";
import Toast from "./toast_item";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { ToastConfig } from "@fluster/desktop_bindings";

const ToastNotificationList = (): ReactNode => {
    const [items, setItems] = useState<(ToastConfig & { createdAt: number })[]>([]);

    useEventListener("show-toast", (e) => {
        setItems([
            ...items,
            {
                ...e.detail,
                createdAt: Date.now(),
            },
        ]);
    });

    return (
        <div
            id="toast-list"
            className="h-fit flex flex-col justify-end items-center gap-6 max-w-[350px] max-h-screen absolute right-0 bottom-0 p-6 overflow-hidden overflow-y-auto"
        >
            {items
                .sort((a, b) => b.createdAt - a.createdAt)
                .map((t: ToastConfig) => (
                    <Toast
                        removeSelf={() => {
                            console.log("removingSelf: ");
                            setItems(items.filter((x: ToastConfig) => x.id !== t.id));
                        }}
                        item={t}
                        key={t.id}
                    />
                ))}
        </div>
    );
};

ToastNotificationList.displayName = "ToastNotificationList";

export default ToastNotificationList;
