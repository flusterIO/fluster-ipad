import { DictionaryData } from "@/code_gen/flat_buffer/dictionary";
import { ByteBuffer } from "flatbuffers";
import { useEffect, useState } from "react";
import { DictionaryWebviewActions, DictionaryWebviewEvents, DictionaryWebviewStorageKeys, sendToSwift, useEventListener } from "src";

declare global {
    interface WindowEventMap {
        [DictionaryWebviewEvents.SetDictionaryData]: CustomEvent<number[]>;
    }
}

export const useDictionaryData = (): DictionaryData | null => {
    const [data, setData] = useState<null | DictionaryData>(null)
    useEventListener(DictionaryWebviewEvents.SetDictionaryData, (e) => {
        try {
            const bytes = new Uint8Array(e.detail)
            const buf = new ByteBuffer(bytes)
            const dictionaryData = DictionaryData.getRootAsDictionaryData(buf)
            setData(dictionaryData)
        } catch (err) {
            console.log("Error: ", err)
        }
    })
    useEffect(() => {
        if (!data) {
            let existingData: string | null = window.localStorage.getItem(DictionaryWebviewStorageKeys.DictionaryData);
            if (existingData) {
                try {
                    let numberArray: number[] = JSON.parse(existingData)
                    let bytes = new Uint8Array(numberArray)
                    const buf = new ByteBuffer(bytes)
                    let d = DictionaryData.getRootAsDictionaryData(buf)
                    setData(d)
                } catch (err) {
                    console.error("An error occurred while deserializing initial dictionary data: ", err)
                }
            }
            sendToSwift(DictionaryWebviewActions.RequestDictionaryData)
        } else {
            sendToSwift(DictionaryWebviewActions.SetWebviewLoaded)
        }
    }, [data])
    return data
}
