
import { Toaster } from "@/components/ui/toaster";
import { Toaster as Sonner } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { DashboardLayout } from "./components/DashboardLayout";
import Dashboard from "./pages/Dashboard";
import VoiceVault from "./pages/VoiceVault";
import MintStudio from "./pages/MintStudio";
import NotFound from "./pages/NotFound";

const queryClient = new QueryClient();

const App = () => (
  <QueryClientProvider client={queryClient}>
    <TooltipProvider>
      <Toaster />
      <Sonner />
      <BrowserRouter>
        <DashboardLayout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/voice-vault" element={<VoiceVault />} />
            <Route path="/mint-studio" element={<MintStudio />} />
            <Route path="/marketplace" element={<div className="text-center py-12"><h2 className="text-2xl font-bold text-primary">Marketplace Coming Soon</h2></div>} />
            <Route path="/analytics" element={<div className="text-center py-12"><h2 className="text-2xl font-bold text-primary">Analytics Coming Soon</h2></div>} />
            <Route path="/daos" element={<div className="text-center py-12"><h2 className="text-2xl font-bold text-primary">DAOs & Collectives Coming Soon</h2></div>} />
            <Route path="/settings" element={<div className="text-center py-12"><h2 className="text-2xl font-bold text-primary">Settings Coming Soon</h2></div>} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </DashboardLayout>
      </BrowserRouter>
    </TooltipProvider>
  </QueryClientProvider>
);

export default App;
