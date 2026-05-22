export interface GeneralPageQuery {
    slug: string[];
    path?: string;
    id?: string;
    topic?: string;
    subject?: string;
    tags?: {
        anyOf?: string[];
        allOf?: string[];
    };
}
