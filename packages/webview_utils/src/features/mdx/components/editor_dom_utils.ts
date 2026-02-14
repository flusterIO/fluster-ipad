import { WebviewEnvironment } from "@/code_gen/typeshare/fluster_core_utilities"

export const setBodyLoading = (loading: boolean) => {
    if (loading) {
        document.body.classList.add("loading")
    } else {
        document.body.classList.remove("loading")
    }
}



export const isWebviewOfEnv = (e: WebviewEnvironment): boolean => {
    return document.body.classList.contains(e)
}
