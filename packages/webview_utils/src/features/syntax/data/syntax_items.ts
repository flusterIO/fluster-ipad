import { aiParsingRequestSyntax } from "./syntax_items/ai_parsing_request";
import { citationSyntax } from "./syntax_items/citation";
import { dictionarySyntax } from "./syntax_items/dictionary_entry";
import { docsSyntax } from "./syntax_items/docs";
import { hrWithChildrenSyntax } from "./syntax_items/hr_with_children";
import { noteLinkSyntax } from "./syntax_items/note_link";
import { tagSyntax } from "./syntax_items/tag";
import { type FlusterSyntax } from "./syntax_types";

export const specialSyntaxItems: FlusterSyntax[] = [
    tagSyntax,
    noteLinkSyntax,
    docsSyntax,
    dictionarySyntax,
    citationSyntax,
    aiParsingRequestSyntax,
    hrWithChildrenSyntax
] as const
