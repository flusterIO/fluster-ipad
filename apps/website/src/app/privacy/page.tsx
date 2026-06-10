import { Button, buttonVariants } from "#/core/shad/ui/button";
import Link from "next/link";
import React, { type ReactNode } from "react";

const FlusterPrivacyPage = (): ReactNode => {
    return (
        <div className="w-full h-fit min-h-screen flex flex-col justify-center items-center">
            <div className="w-full max-w-[450px] h-fit rounded-lg border p-3 m-4">
                <h3 className="font-bold text-lg">Privacy</h3>
                <div className="w-full">
                    With <span className="font-bold">Fluster</span>, your data never
                    touches a database that is not operated by Apple or ran on your own
                    device. All data is managed by Apple's SwiftData framework, and stored
                    on servers owned and operated by Apple.
                </div>
                <div className="w-full flex flex-row justify-end items-center mt-2">
                    <Link href="/" className={buttonVariants()}>
                        Back to Fluster
                    </Link>
                </div>
            </div>
        </div>
    );
};

FlusterPrivacyPage.displayName = "FlusterPrivacyPage";

export default FlusterPrivacyPage;
