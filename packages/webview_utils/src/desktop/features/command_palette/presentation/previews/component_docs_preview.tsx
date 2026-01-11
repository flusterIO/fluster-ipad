import { MdxContent } from "#/mdx/components/mdx_content";
import { MdxProviderGroup } from "#/mdx/components/mdx_provider_group";
import { commands } from "@fluster/desktop_bindings";
import React, { ReactNode, useEffect, useEffectEvent, useState } from "react";
import { CommandPaletteAnyEntry } from "../../data/command_palette_any_entry";

export const ComponentDocsPreview = ({
    item,
}: {
    item: CommandPaletteAnyEntry;
}): ReactNode => {
    const [mdxContent, setMdxContent] = useState<null | string>(null);
    const getData = useEffectEvent(async (fsPath: string): Promise<void> => {
        const res = await commands.getEmbeddedDocByRelativePath(fsPath);
        if (res.status === "ok") {
            setMdxContent(res.data);
        } else {
            console.error("Could not get mdx content.");
        }
    })
    useEffect(() => {
        if (!item) {
            return;
        }
        if ("previewPath" in item) {
            getData(item.previewPath as string);
        }
    }, [item]);
    if (!mdxContent) {
        return null;
    }
    return (
        <div className="overflow-y-auto max-h-[70vh]">
            <MdxProviderGroup>
                <MdxContent className="mdx-small" mdx={mdxContent} />
            </MdxProviderGroup>
        </div>
    );
};
