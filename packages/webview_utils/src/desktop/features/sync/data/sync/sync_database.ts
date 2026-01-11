export interface SyncDatabaseArgs {
    with_ai: boolean
    showSuccessToast: boolean
}


export const syncDatabase = async (args: SyncDatabaseArgs) => {
    console.log("args: ", args)

}
