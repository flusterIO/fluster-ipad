import React, { type ReactNode } from "react";
import { isRouteErrorResponse, useRouteError } from "react-router";

interface RouteErrorBoundaryProps {}

export const RouteErrorBoundary = (
  props: RouteErrorBoundaryProps,
): ReactNode => {
  const error = useRouteError();
  if (isRouteErrorResponse(error)) {
    if (error.status === 404) {
      return <RouteError404 />;
    }
  }
  return <div>Something wewn't wrong</div>;
};

RouteErrorBoundary.displayName = "RouteErrorBoundary";
