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
import Marketplace from "./pages/Marketplace";
import Analytics from "./pages/Analytics";
import DAOs from "./pages/DAOs";
import Settings from "./pages/Settings";

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
            <Route path="/marketplace" element={<Marketplace />} />
            <Route path="/analytics" element={<Analytics />} />
            <Route path="/daos" element={<DAOs />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </DashboardLayout>
      </BrowserRouter>
    </TooltipProvider>
  </QueryClientProvider>
);

export default App;
