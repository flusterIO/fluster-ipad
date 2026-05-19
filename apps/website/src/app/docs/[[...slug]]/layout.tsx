import type { ReactNode } from "react";
import "../../../core/styles/docs.scss";
import { PermanentLeftSidebar } from "@conundrum/ts/ui/blog-ssr";
import { FunctionSquare, NotebookTabs } from "lucide-react";
import { ToggleSidebarClientButton } from "#/features/blog/sidebar/toggle_sidebar_client_button";

export default function Layout({ children }: { children: ReactNode }) {
    return (
        <div
            data-cdrm-sidebar="open"
            className="w-full h-fit flex flex-col justify-center items-center"
        >
            <PermanentLeftSidebar
                topics={[
                    {
                        icon: (props) => <NotebookTabs {...props} />,
                        label: "Documentation",
                    },
                    {
                        icon: (props) => <FunctionSquare  {...props} />,
                        label: "My Work",
                    },
                ]}
                toggleButton={<ToggleSidebarClientButton />}
            />
            {children}
        </div>
    );
}
