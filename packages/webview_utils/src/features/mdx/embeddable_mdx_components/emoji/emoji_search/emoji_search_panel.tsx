import { InputGroup, InputGroupAddon, InputGroupInput } from '@/shared_components/shad/input-group'
import { SearchIcon } from 'lucide-react'
import React, { useEffect, useRef, useState, type ReactNode } from 'react'
import { EmojiDocsDemo } from '../emoji_docs_demo'
import { type EmojiData } from '@/code_gen/typeshare/conundrum'
import { search_conundrum_emojis } from "@fluster/wasm"

const DEBOUNCE_TIMEOUT = 500;
const PAGE = 1;
const PER_PAGE = 50;

export const EmojiSearchPanel = (): ReactNode => {
    const [emojis, setEmojis] = useState<EmojiData[]>([])
    const debounce = useRef<NodeJS.Timeout | null>(null);
    const [query, setQuery] = useState("")
    useEffect(() => {
        if (debounce.current) {
            clearTimeout(debounce.current)
        }
        debounce.current = setTimeout(() => {
            const r = search_conundrum_emojis(query, PAGE, PER_PAGE) as EmojiData[];
            setEmojis(r);
        }, DEBOUNCE_TIMEOUT)
    }, [query])
    return (
        <div className="flex flex-col justify-center items-center gap-y-4">
            <div className="w-full flex flex-row justify-end items-center gap-4">
                <InputGroup>
                    <InputGroupAddon >
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputGroupInput value={query} onChange={(e) => { setQuery(e.target.value); }} />
                </InputGroup>
            </div>
            <div
                className="grid w-full gap-3"
                style={{
                    display: "grid",
                    gridTemplateColumns: "repeat(auto-fit, minmax(150px, 1fr))"
                }}
            >
                {emojis.map((e) => {
                    return (
                        <EmojiDocsDemo data={e} key={e.name} />
                    )
                })}
            </div>
        </div>
    )
}


EmojiSearchPanel.displayName = "EmojiSearchPanel"
