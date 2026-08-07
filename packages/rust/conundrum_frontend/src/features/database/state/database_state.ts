export interface DBAuth {
    /**
     *  This is completely just a useless placeholder right now to get the DB state in place. It does nothing.
     */
    user_name: string;
}

export interface DatabaseState {
    auth: DBAuth;
}
