import { nestedContent } from "../../../../../.source/";
import { loader } from "fumadocs-core/source";

export const nestedContentSource = loader({
    baseUrl: "/",
    source: nestedContent.toFumadocsSource(),
});
