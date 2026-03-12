import { type DependencyList, useEffect, useRef } from "react"

export const useDebounce = (callback: () => void, deps: DependencyList, debounce = 500) => {
    const timer = useRef<NodeJS.Timeout | null>(null)
    useEffect(() => {
        if (timer.current) {
            clearTimeout(timer.current)
        }
        timer.current = setTimeout(callback, debounce)
    }, deps)
}
