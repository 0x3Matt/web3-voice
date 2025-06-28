import { SidebarProvider, SidebarInset, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "./AppSidebar";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export const DashboardLayout = ({ children }: DashboardLayoutProps) => {
  return (
    <SidebarProvider>
      <div className="min-h-screen flex w-full">
        <AppSidebar />
        <SidebarInset className="flex-1 overflow-auto">
          {/* Mobile header with menu button */}
          <header className="flex h-12 items-center border-b border-border/40 px-4 md:hidden bg-background/80 backdrop-blur-sm sticky top-0 z-10">
            <SidebarTrigger className="mr-2" />
            <div className="flex items-center space-x-2">
              <img 
                src="/favicon-32x32.png" 
                alt="Web3Voice Logo" 
                className="w-6 h-6" 
              />
              <h1 className="text-lg font-semibold text-primary">Web3Voice</h1>
            </div>
          </header>
          
          <div className="p-3 sm:p-4 md:p-6 lg:p-8">
            {children}
          </div>
        </SidebarInset>
      </div>
    </SidebarProvider>
  );
};
