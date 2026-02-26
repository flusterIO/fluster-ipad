import { Emphasis } from "../schemas/emphasis_schema";

export const getTitleVariantClasses = (variant: Emphasis): string => {
    switch (variant) {
        case "success":
            return "bg-emphasis-success text-emphasis-success-foreground [&_p]:text-emphasis-success-foreground";
        case "info":
            return "bg-emphasis-info text-emphasis-info-foreground [&_p]:text-emphasis-info-foreground";
        case "error":
            return "bg-emphasis-error text-emphasis-error-foreground [&_p]:text-emphasis-error-foreground";
        case "warn":
            return "bg-emphasis-warn text-emphasis-warn-foreground [&_p]:text-emphasis-warn-foreground";
        case "primary":
            return "bg-primary text-primary-foreground [&_p]:text-primary-foreground";
        case "research":
            return "bg-emphasis-research text-emphasis-research-foreground [&_p]:text-emphasis-research-foreground"
        case "important":
            return "bg-emphasis-important text-emphasis-important-foreground [&_p]:text-emphasis-important-foreground"
        case "highlight":
            return "bg-emphasis-highlight text-emphasis-highlight-foreground [&_p]:text-emphasis-highlight-foreground"
    }
};
