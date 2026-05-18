export interface GeneralPageQuery {
    slug: string[];
    path?: string;
    id?: string;
    keywords?: {
        anyOf?: string[];
        allOf?: string[];
    };
}
