import React, { type ReactNode } from "react";
import { Label } from "@/components/shad/label";
import { cn } from "@/utils/shad_utils";

interface DisabledLabeledTextProps {
  label: ReactNode;
  content: string;
  classes?: {
    container?: string;
    content?: string;
  };
  /**
   * Set to true if it represents a file system path or code to apply the mono syles.
   */
  mono?: boolean;
}

export const DisabledLabeledText = ({
  label,
  content,
  mono,
  classes = {},
}: DisabledLabeledTextProps): ReactNode => {
  return (
    <div
      className={cn(
        "flex flex-col justify-center items-start gap-y-2",
        classes.container,
      )}
    >
      <Label>{label}</Label>
      <div
        className={cn(
          "rounded-sm p-1 border bg-muted/50 text-foreground/80",
          mono && "font-mono",
          classes.content,
        )}
      >
        {content}
      </div>
    </div>
  );
};

DisabledLabeledText.displayName = "DisabledLabeledText";
