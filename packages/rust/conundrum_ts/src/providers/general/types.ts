export interface GeneralPageQuery {
    path?: string;
    id?: string;
    keywords?: {
        anyOf?: string[];
        allOf?: string[];
    };
}
