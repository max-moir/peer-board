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
        label: "Chat",
        href: "/",
        icon: BarChartSquare02,
      },
      {
        label: "About",
        href: "/challenge",
        icon: File05,
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
