import consola from "consola";
import { useEffect, useState } from "react"


export const useLocalLoadingState = <T>(cb: (() => Promise<T>), initialLoading = false): [T | undefined, boolean] => {
    const [value, setValue] = useState<T>()
    const [loading, setLoading] = useState(initialLoading);

    const getData = async (): Promise<void> => {
        setLoading(true)
        const res: T = await cb()
        setValue(res)
        setLoading(false)
    }
    useEffect(() => {
        getData().catch((err: unknown) => { consola.error(`Error: ${err}`); })
    }, [])

    return [value, loading]

    }
