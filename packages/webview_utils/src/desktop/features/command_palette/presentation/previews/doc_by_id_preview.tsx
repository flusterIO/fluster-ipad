import React, { ReactNode, useEffect, useEffectEvent, useState } from "react";
import { CommandPaletteAnyEntry } from "../../data/command_palette_any_entry";
import { MdxContent } from "#/mdx/components/mdx_content";
import { MdxProviderGroup } from "#/mdx/components/mdx_provider_group";
import { InternalEmbeddedDocsId, commands } from "@fluster/desktop_bindings";

export const DocsByIdPreview = ({
    item,
}: {
    item: CommandPaletteAnyEntry;
}): ReactNode => {
    const [mdxContent, setMdxContent] = useState<null | string>(null);
    const getData = useEffectEvent(async (docId: InternalEmbeddedDocsId): Promise<void> => {
        const res = await commands.getEmbeddedDoc(docId);
        setMdxContent(res);
    })

    useEffect(() => {
        if (!item) {
            return;
        }
        if ("docId" in item) {
            getData(item.docId as InternalEmbeddedDocsId);
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
