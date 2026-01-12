import { AppRoutes } from "../features/navigation/data/app_routes";

export const getTabularDataTableUrl = (relativeFilePath: string) => {
    const sp = new URLSearchParams();
    sp.set("file", relativeFilePath);
    return `${AppRoutes.tabular_data_table}?${sp.toString()}`;
};


export const getTopicUrl = (topicValue: string): string => {
    const sp = new URLSearchParams();
    sp.set("by_topic", topicValue);
    return `${AppRoutes.search}?${sp.toString()}`;
};

export const getSubjectUrl = (subjectValue: string): string => {
    const sp = new URLSearchParams();
    sp.set("by_subject", subjectValue);
    return `${AppRoutes.search}?${sp.toString()}`;
};


export const getMdxNoteUrl = (fsPath: string): string => {
    const sp = new URLSearchParams();
    sp.set("fsPath", fsPath);
    return `${AppRoutes.viewMdxNote}?${sp.toString()}`;
};


export const getPdfUrl = (pdfPath: string): string => {
    const sp = new URLSearchParams();
    sp.set("fsPath", pdfPath);
    return `${AppRoutes.pdf}?${sp.toString()}`;
};


export const getHtmlFileURl = (pdfPath: string): string => {
    const sp = new URLSearchParams();
    sp.set("fsPath", pdfPath);
    return `${AppRoutes.htmlFile}?${sp.toString()}`;
};


export const getEmbeddedDocUrl = (docId: string) =>
    `${AppRoutes.embeddedDocs.toString()}/${encodeURI(docId)}`;

export const getByEquationUrl = (equationId: string): string => {
    const sp = new URLSearchParams();
    sp.set("by_equation", equationId);
    return `${AppRoutes.search}?${sp.toString()}`;
};
