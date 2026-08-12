import React, { type ReactNode } from "react";

interface FilePathCellProps {
  children: ReactNode;
}

export const FilePathCell = ({ children }: FilePathCellProps): ReactNode => {
  return <div className="text-sm font-mono">{children}</div>;
};

FilePathCell.displayName = "FilePathCell";
