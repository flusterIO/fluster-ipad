import { type TableHealthReport } from "@/codegen/bindings";
import { CheckIcon, XIcon, XSquareIcon } from "lucide-react";
import React, { useState, type ReactNode } from "react";
import { motion } from "framer-motion";
import { CdrmContent } from "#/cdrm/cdrm_content";

export const DatabaseTableHealthItem = ({
    item,
}: {
    item: TableHealthReport;
}): ReactNode => {
    const [open, setOpen] = useState(false);
    return (
        <div className="w-full h-fit rounded p-4 border bg-fd-card text-fd-card-foreground">
            <div
                className="grid grid-cols-[auto_1fr] gap-x-2"
                onClick={() => {
                    setOpen(!open);
                }}
            >
                <div className="place-self-center">
                    {item.exists ? (
                        <div className="bg-green-500 rounded">
                            <CheckIcon className="size-5" />
                        </div>
                    ) : (
                        <div className="bg-red-600 rounded">
                            <XIcon className="size-5" />
                        </div>
                    )}
                </div>
                <h4 className="text-lg font-bold">{item.description.entity_name}</h4>
            </div>
            <motion.div
                /* className="" */
                variants={{
                    open: {
                        height: "auto",
                        marginTop: "1rem",
                    },
                    close: {
                        height: 0,
                        marginTop: 0,
                    },
                }}
                initial="close"
                animate={open ? "open" : "close"}
                className="overflow-hidden"
            >
                <h6 className="font-bold">AI Instructions</h6>
                <CdrmContent
                    className="text-fd-card-foreground/80"
                    content={item.description.description}
                />
            </motion.div>
        </div>
    );
};

DatabaseTableHealthItem.displayName = "DatabaseTableHealthItem";
