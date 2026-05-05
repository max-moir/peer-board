import { BarChartSquare02, File05 } from "@untitledui/icons";
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
        label: "Battleship",
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
