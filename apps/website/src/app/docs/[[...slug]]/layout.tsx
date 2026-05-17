import type { ReactNode } from "react";
import "../../../core/styles/docs.scss";

export default function Layout({ children }: { children: ReactNode }) {
    return (
        <div className="w-full h-fit flex flex-col justify-center items-center">
            {children}
        </div>
    );
}
