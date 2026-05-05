import React from "react";
import { NavItemBase } from "../base-components/nav-item";
import { ThemeToggle } from "@/components/base/theme-toggle";
import type { NavItemType } from "../config";

interface SidebarNavigationSectionsSubheadingsProps {
  activeUrl?: string;
  items: Array<{ label: string; items: NavItemType[] }>;
}

export const SidebarNavigationSectionsSubheadings = ({
  activeUrl = "/",
  items,
}: SidebarNavigationSectionsSubheadingsProps) => {
  const MAIN_SIDEBAR_WIDTH = 292;

  const [channels, setChannels] = React.useState<string[]>([
    "general",
    "random",
  ]);

  const addChannel = () => {
    const name = prompt("Channel name?");
    if (!name) return;
    if (!name || !/^[a-z0-9-]+$/.test(name)) {
      alert(
        "Channel name must only contain lowercase letters, numbers, and hyphens.",
      );
      return;
    }

    const trimmed = name.trim();
    if (!trimmed) return;

    // prevent duplicates
    if (channels.includes(trimmed)) return;

    setChannels((prev) => [...prev, trimmed]);
  };

  const removeChannel = (name: string) => {
    setChannels((prev) => prev.filter((c) => c !== name));
  };

  const content = (
    <aside
      style={
        {
          "--width": `${MAIN_SIDEBAR_WIDTH}px`,
        } as React.CSSProperties
      }
      className="flex h-full w-full max-w-full flex-col justify-between overflow-auto border-secondary bg-primary pt-4 shadow-xs md:border-r lg:w-(--width) lg:border lg:pt-5"
    >
      {/* Header */}
      <div className="flex flex-col gap-5 px-4 lg:px-5">
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold text-primary">PeerBoard</h1>

          <ThemeToggle />
        </div>
      </div>

      {/* Navigation */}
      <ul className="mt-8">
        {/* 🔹 Existing General Sections */}
        {items.map((group) => (
          <li key={group.label}>
            <div className="px-5 pb-1">
              <p className="text-xs font-bold text-quaternary uppercase">
                {group.label}
              </p>
            </div>
            <ul className="px-4 pb-5">
              {group.items.map((item) => (
                <li key={item.label} className="py-0.5">
                  <NavItemBase
                    icon={item.icon}
                    href={item.href}
                    badge={item.badge}
                    type="link"
                    current={item.href === activeUrl}
                  >
                    {item.label}
                  </NavItemBase>
                </li>
              ))}
            </ul>
          </li>
        ))}

        {/* 🔹 Channels Section */}
        {/* <li>
          <div className="flex items-center justify-between px-5 pb-1">
            <p className="text-xs font-bold text-quaternary uppercase">
              Channels
            </p>
            <button
              onClick={addChannel}
              className="text-xs text-tertiary hover:text-primary"
            >
              +
            </button>
          </div>

          <ul className="px-4 pb-5">
            {channels.map((channel) => {
              const href = `/topics/${channel}`;
              return (
                <li
                  key={channel}
                  className="group flex items-center justify-between py-0.5"
                >
                  <NavItemBase
                    href={href}
                    type="link"
                    current={href === activeUrl}
                  >
                    # {channel}
                  </NavItemBase>

                  <button
                    onClick={() => removeChannel(channel)}
                    className="invisible text-xs text-tertiary hover:text-primary group-hover:visible"
                  >
                    ×
                  </button>
                </li>
              );
            })}
          </ul>
        </li> */}
      </ul>

      {/* Footer */}
      <div className="mt-auto flex flex-col gap-5 px-2 py-4 lg:gap-6 lg:px-4 lg:py-4"></div>
    </aside>
  );

  return (
    <>
      {/* Desktop sidebar */}
      <div className="hidden lg:fixed lg:inset-y-0 lg:left-0 lg:flex">
        {content}
      </div>

      {/* Spacer */}
      <div
        style={{
          paddingLeft: MAIN_SIDEBAR_WIDTH + 0,
        }}
        className="invisible hidden lg:sticky lg:top-0 lg:bottom-0 lg:left-0 lg:block"
      />
    </>
  );
};
