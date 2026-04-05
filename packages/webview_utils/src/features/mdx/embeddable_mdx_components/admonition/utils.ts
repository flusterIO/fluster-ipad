import { Emphasis } from "../schemas/emphasis_schema";

export const getTitleVariantClasses = (variant: Emphasis): string => {
    switch (variant) {
        case Emphasis.Success:
            return "bg-emphasis-success text-emphasis-success-foreground [&_*]:text-emphasis-success-foreground";
        case Emphasis.Info:
            return "bg-emphasis-info text-emphasis-info-foreground [&_*]:text-emphasis-info-foreground";
        case Emphasis.Error:
            return "bg-emphasis-error text-emphasis-error-foreground [&_*]:text-emphasis-error-foreground";
        case Emphasis.Warn:
            return "bg-emphasis-warn text-emphasis-warn-foreground [&_*]:text-emphasis-warn-foreground";
        case Emphasis.Primary:
            return "bg-primary text-primary-foreground [&_*]:text-primary-foreground";
        case Emphasis.Research:
            return "bg-emphasis-research text-emphasis-research-foreground [&_*]:text-emphasis-research-foreground"
        case Emphasis.Important:
            return "bg-emphasis-important text-emphasis-important-foreground [&_*]:text-emphasis-important-foreground"
        case Emphasis.Highlight:
            return "bg-emphasis-highlight text-emphasis-highlight-foreground [&_*]:text-emphasis-highlight-foreground"
        case Emphasis.Card:
            return "bg-fd-card text-fd-card-foreground [&_*]:text-fd-card-foreground"
    }
};
