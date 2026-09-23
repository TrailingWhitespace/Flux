"use client";

import Link from "next/link";

import { usePathname } from "next/navigation";
import { motion } from "framer-motion";
import {
  LayoutDashboard,
  CheckSquare,
  StickyNote,
  FolderKanban,
} from "lucide-react";
import "./sidebar.css";

const NAV_ITEMS = [
  { href: "/", label: "Home", icon: LayoutDashboard },
  { href: "/todos", label: "Todos", icon: CheckSquare },
  { href: "/notes", label: "Notes", icon: StickyNote },
  { href: "/projects", label: "Projects", icon: FolderKanban },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <span className="sidebar-title">Flux</span>
      </div>

      <nav className="sidebar-nav">
        {NAV_ITEMS.map(({ href, label, icon: Icon }) => {
          const isActive = pathname === href;
          return (
            <Link
              key={href}
              href={href}
              className={`sidebar-link ${isActive ? "sidebar-link-active" : ""}`}
            >
              {isActive && (
                <motion.div
                  layoutId="active-indicator"
                  className="sidebar-active-pill"
                  transition={{
                    type: "spring",
                    stiffness: 380,
                    damping: 30,
                  }}
                />
              )}
              <Icon size={20} strokeWidth={1.75} className="sidebar-icon" />
              <span>{label}</span>
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
