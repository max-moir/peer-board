import {
  BarChartSquare02,
  Calendar,
  CheckDone01,
  ChevronRight,
  File05,
  PieChart03,
  Rows01,
  Users01,
} from "@untitledui/icons";
import type { NavItemType } from "@/components/application/app-navigation/config";
import { SidebarNavigationSectionsSubheadings } from "@/components/application/app-navigation/sidebar-navigation/sidebar-sections-subheadings";

const navItemsWithSectionsSubheadings: Array<{
  label: string;
  items: NavItemType[];
}> = [
  {
    label: "General",
    items: [
      {
        label: "Dashboard",
        href: "/",
        icon: BarChartSquare02,
      },
      {
        label: "Projects",
        href: "/projects",
        icon: Rows01,
      },
      {
        label: "Documents",
        href: "/documents",
        icon: File05,
      },
      {
        label: "Calendar",
        href: "/calendar",
        icon: Calendar,
      },
    ],
  },
];

export const SidebarNavigationSectionsSubheadingsDemo = () => (
  <SidebarNavigationSectionsSubheadings
    activeUrl="/"
    items={navItemsWithSectionsSubheadings}
  />
);
