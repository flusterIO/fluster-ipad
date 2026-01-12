import React, { type ReactNode } from "react";
import { ModularDashboard } from "./dashboards/modular_dashboard/modular_dashboard";

interface DashboardSwitchProps {}

export const DashboardSwitch = (props: DashboardSwitchProps): ReactNode => {
  return <ModularDashboard />;
};

DashboardSwitch.displayName = "DashboardSwitch";
