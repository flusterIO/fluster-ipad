import { type Emphasis } from "../schemas/emphasis_schema";

export const getTitleVariantClasses = (variant: Emphasis): string => {
    switch (variant) {
        case "success":
            return "bg-emphasis-success text-emphasis-success-foreground [&_*]:text-emphasis-success-foreground";
        case "info":
            return "bg-emphasis-info text-emphasis-info-foreground [&_*]:text-emphasis-info-foreground";
        case "error":
            return "bg-emphasis-error text-emphasis-error-foreground [&_*]:text-emphasis-error-foreground";
        case "warn":
            return "bg-emphasis-warn text-emphasis-warn-foreground [&_*]:text-emphasis-warn-foreground";
        case "primary":
            return "bg-primary text-primary-foreground [&_*]:text-primary-foreground";
        case "research":
            return "bg-emphasis-research text-emphasis-research-foreground [&_*]:text-emphasis-research-foreground"
        case "important":
            return "bg-emphasis-important text-emphasis-important-foreground [&_*]:text-emphasis-important-foreground"
        case "highlight":
            return "bg-emphasis-highlight text-emphasis-highlight-foreground [&_*]:text-emphasis-highlight-foreground"
        case "card":
            return "bg-fd-card text-fd-card-foreground [&_*]:text-fd-card-foreground"
    }
};
