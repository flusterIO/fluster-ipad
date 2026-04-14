import { InputGroup, InputGroupAddon, InputGroupInput } from '@/shared_components/shad/input-group'
import { ChevronLeft, ChevronRight, SearchIcon } from 'lucide-react'
import React, { useEffect, useRef, useState, type ReactNode } from 'react'
import { EmojiDocsDemo } from '../emoji_docs_demo'
import { type EmojiSearchResults } from '@/code_gen/typeshare/conundrum'
import { search_conundrum_emojis } from "@fluster/wasm"
import { Button } from '@/shared_components/shad/button'

const DEBOUNCE_TIMEOUT = 150;
const PER_PAGE = 50;

export const EmojiSearchPanel = (): ReactNode => {
    const [emojis, setEmojis] = useState<EmojiSearchResults>({
        names: [],
        total: 0
    });
    const [page, setPage] = useState(1)
    const debounce = useRef<NodeJS.Timeout | null>(null);
    const [query, setQuery] = useState("")
    useEffect(() => {
        if (debounce.current) {
            clearTimeout(debounce.current)
        }
        debounce.current = setTimeout(() => {
            const r = search_conundrum_emojis(query, page, PER_PAGE) as EmojiSearchResults;
            setEmojis(r);
        }, DEBOUNCE_TIMEOUT)
    }, [query, page])
    return (
        <div className="flex flex-col justify-center items-center gap-y-4">
            <div className="w-full flex flex-row justify-end items-center gap-4">
                <InputGroup>
                    <InputGroupAddon >
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputGroupInput value={query} onChange={(e) => {
                        setPage(1);
                        setQuery(e.target.value);
                    }} />
                </InputGroup>
            </div>
            <div
                className="grid w-full gap-3"
                style={{
                    display: "grid",
                    gridTemplateColumns: "repeat(auto-fit, minmax(150px, 1fr))"
                }}
            >
                {emojis.names.map((e) => {
                    return (
                        <EmojiDocsDemo data={e} key={e.name} />
                    )
                })}
            </div>
            <EmojiDocsPagination setPage={setPage} total={emojis.total} page={page} />
        </div>
    )
}


interface EmojiDocsPaginationProps {
    total: number;
    page: number;
    setPage: (p: number) => void
}

const EmojiDocsPagination = ({ total, page, setPage }: EmojiDocsPaginationProps): ReactNode => {
    if (total <= PER_PAGE) {
        return null
    }
    const startIndex = page * PER_PAGE;
    const endIndex = startIndex + PER_PAGE;
    return (
        <div className="w-full h-fit flex flex-row justify-end items-center gap-x-2">
            <Button
                disabled={page <= 1}
                size={"icon-sm"}
                onClick={() => { setPage(page - 1); }}
                variant={"outline"}
            >
                <ChevronLeft
                    size={"text-sm"}
                />
            </Button>
            <Button
                disabled={endIndex >= total}
                size={"icon-sm"}
                onClick={() => { setPage(page + 1); }}
                variant={"outline"}
            >
                <ChevronRight className="text-sm" />
            </Button>
        </div>
    )
}


EmojiSearchPanel.displayName = "EmojiSearchPanel"
