import { Home, Mic, Coins, ShoppingCart, BarChart, Users, Settings } from "lucide-react";
import { NavLink, useLocation } from "react-router-dom";

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";

const items = [
  { title: "Dashboard", url: "/", icon: Home },
  { title: "Voice Vault", url: "/voice-vault", icon: Mic },
  { title: "Mint Studio", url: "/mint-studio", icon: Coins },
  { title: "Marketplace", url: "/marketplace", icon: ShoppingCart },
  { title: "Analytics", url: "/analytics", icon: BarChart },
  { title: "DAOs", url: "/daos", icon: Users },
  { title: "Settings", url: "/settings", icon: Settings },
];

export function AppSidebar() {
  const { state } = useSidebar();
  const location = useLocation();
  const currentPath = location.pathname;
  const isCollapsed = state === "collapsed";

  const isActive = (path: string) => currentPath === path;

  return (
    <Sidebar className="border-r border-primary/20" collapsible="icon">
      <SidebarContent className="bg-sidebar">
        <SidebarGroup>
          <SidebarGroupLabel className="text-primary font-bold text-base sm:text-lg px-4 py-3">
            <div className="flex items-center space-x-2">
              <img 
                src="/favicon-32x32.png" 
                alt="Web3Voice Logo" 
                className="w-6 h-6 sm:w-7 sm:h-7 shrink-0" 
              />
              {!isCollapsed && (
                <span className="truncate">Web3Voice</span>
              )}
            </div>
          </SidebarGroupLabel>
          
          <SidebarGroupContent>
            <SidebarMenu className="space-y-1 px-2">
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton
                    asChild
                    isActive={isActive(item.url)}
                    className={`
                      w-full flex items-center space-x-3 px-3 py-2.5 rounded-lg transition-all duration-200
                      ${isActive(item.url) 
                        ? 'bg-primary/20 text-primary border border-primary/30 glow-primary' 
                        : 'hover:bg-muted/20 text-muted-foreground hover:text-foreground'
                      }
                      ${isCollapsed ? 'justify-center' : 'justify-start'}
                    `}
                  >
                    <NavLink to={item.url} className="flex items-center space-x-3 w-full">
                      <item.icon className="w-4 h-4 sm:w-5 sm:h-5 shrink-0" />
                      {!isCollapsed && (
                        <span className="text-sm sm:text-base font-medium truncate">
                          {item.title}
                        </span>
                      )}
                    </NavLink>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
