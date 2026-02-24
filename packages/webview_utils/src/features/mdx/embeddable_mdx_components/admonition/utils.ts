import { Emphasis } from "../schemas/emphasis_schema";

export const getTitleVariantClasses = (variant: Emphasis): string => {
    switch (variant) {
        case "success":
            return "bg-emphasisSuccess text-emphasisSuccessForeground [&_p]:text-emphasisSuccessForeground";
        case "info":
            return "bg-emphasisInfo text-emphasisInfoForeground [&_p]:text-emphasisInfoForeground";
        case "error":
            return "bg-emphasisError text-emphasisErrorForeground [&_p]:text-emphasisErrorForeground";
        case "warn":
            return "bg-emphasisWarn text-emphasisWarnForeground [&_p]:text-emphasisWarnForeground";
        case "primary":
            return "bg-primary text-primary-foreground [&_p]:text-primary-foreground";
        case "research":
            return "bg-emphasisResearch text-emphasisResearchForeground [&_p]:text-emphasisResearchForeground"
        case "important":
            return "bg-emphasisImportant text-emphasisImportantForeground [&_p]:text-emphasisImportantForeground"
        case "highlight":
            return "bg-emphasisHighlight text-emphasisHighlightForeground [&_p]:text-emphasisHighlightForeground"
    }
};
