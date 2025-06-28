
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import {
  Mic,
  Upload,
  DollarSign,
  Globe,
  Activity,
  TrendingUp,
  Play,
  ExternalLink
} from "lucide-react";

const Dashboard = () => {
  const recentUploads = [
    {
      id: 1,
      title: "Morning Meditation Guide",
      status: "NFT",
      duration: "3:24",
      txId: "0xa1b2c3d4...",
      waveform: [0.2, 0.8, 0.6, 0.9, 0.4, 0.7, 0.3, 0.8, 0.5, 0.9]
    },
    {
      id: 2,
      title: "Crypto Market Analysis",
      status: "Processing",
      duration: "12:45",
      txId: "Pending...",
      waveform: [0.4, 0.6, 0.8, 0.5, 0.9, 0.3, 0.7, 0.6, 0.8, 0.4]
    },
    {
      id: 3,
      title: "Story Chapter 1",
      status: "Draft",
      duration: "8:12",
      txId: "Not minted",
      waveform: [0.6, 0.4, 0.9, 0.7, 0.3, 0.8, 0.5, 0.9, 0.2, 0.7]
    }
  ];

  const networkData = [
    { country: "United States", listeners: 1234, percentage: 45 },
    { country: "United Kingdom", listeners: 678, percentage: 25 },
    { country: "Canada", listeners: 456, percentage: 17 },
    { country: "Australia", listeners: 234, percentage: 13 }
  ];

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Dashboard
          </h1>
          <p className="text-muted-foreground mt-2">
            Welcome back to your Web3Voice dashboard
          </p>
        </div>
        <Button className="token-button">
          <Upload className="w-4 h-4 mr-2" />
          Quick Upload
        </Button>
      </div>

      {/* Top Row - Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card className="cyber-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-muted-foreground">Total Earnings</p>
              <p className="text-2xl font-bold text-primary">$12,458</p>
              <p className="text-xs text-green-400 flex items-center">
                <TrendingUp className="w-3 h-3 mr-1" />
                +23% this month
              </p>
            </div>
            <div className="w-12 h-12 bg-primary/20 rounded-xl flex items-center justify-center glow-primary">
              <DollarSign className="w-6 h-6 text-primary" />
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-muted-foreground">Voice NFTs</p>
              <p className="text-2xl font-bold text-primary">24</p>
              <p className="text-xs text-blue-400">3 pending mint</p>
            </div>
            <div className="w-12 h-12 bg-accent/20 rounded-xl flex items-center justify-center glow-secondary">
              <Mic className="w-6 h-6 text-accent" />
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-muted-foreground">Total Listeners</p>
              <p className="text-2xl font-bold text-primary">2,847</p>
              <p className="text-xs text-purple-400">Across 12 countries</p>
            </div>
            <div className="w-12 h-12 bg-purple-500/20 rounded-xl flex items-center justify-center">
              <Globe className="w-6 h-6 text-purple-400" />
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-muted-foreground">Engagement</p>
              <p className="text-2xl font-bold text-primary">94%</p>
              <p className="text-xs text-emerald-400">Avg. completion rate</p>
            </div>
            <div className="w-12 h-12 bg-emerald-500/20 rounded-xl flex items-center justify-center">
              <Activity className="w-6 h-6 text-emerald-400" />
            </div>
          </div>
        </Card>
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Voice Identity Card */}
        <Card className="cyber-card lg:col-span-1">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold">Voice Identity</h3>
              <Badge className="bg-primary/20 text-primary border-primary/30">Verified</Badge>
            </div>
            
            <div className="flex items-center space-x-4">
              <div className="w-16 h-16 bg-gradient-to-br from-primary to-accent rounded-full flex items-center justify-center glow-primary">
                <Mic className="w-8 h-8 text-white" />
              </div>
              <div>
                <h4 className="font-semibold">Voice Creator Pro</h4>
                <p className="text-sm text-muted-foreground">ID: VC-2024-001</p>
                <Badge variant="secondary" className="mt-1">Tier 3 Creator</Badge>
              </div>
            </div>

            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>Voice Signature Match</span>
                <span className="text-primary">98.7%</span>
              </div>
              <Progress value={98.7} className="h-2" />
            </div>
          </div>
        </Card>

        {/* Recent Uploads */}
        <Card className="cyber-card lg:col-span-2">
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold">Recent Uploads</h3>
              <Button variant="ghost" size="sm">View All</Button>
            </div>

            <div className="space-y-4">
              {recentUploads.map((upload) => (
                <div key={upload.id} className="flex items-center space-x-4 p-4 rounded-lg bg-muted/30 hover:bg-muted/50 transition-colors">
                  <div className="flex items-center space-x-2">
                    {upload.waveform.map((height, i) => (
                      <div
                        key={i}
                        className="w-1 bg-primary/60 rounded-full waveform-pulse"
                        style={{ 
                          height: `${height * 24}px`,
                          animationDelay: `${i * 0.1}s`
                        }}
                      />
                    ))}
                  </div>
                  
                  <div className="flex-1 min-w-0">
                    <h4 className="font-medium truncate">{upload.title}</h4>
                    <div className="flex items-center space-x-4 text-sm text-muted-foreground">
                      <span>{upload.duration}</span>
                      <Badge 
                        variant={upload.status === "NFT" ? "default" : upload.status === "Processing" ? "secondary" : "outline"}
                        className={upload.status === "NFT" ? "bg-primary/20 text-primary" : ""}
                      >
                        {upload.status}
                      </Badge>
                    </div>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Button size="sm" variant="ghost">
                      <Play className="w-4 h-4" />
                    </Button>
                    {upload.status === "NFT" && (
                      <Button size="sm" variant="ghost">
                        <ExternalLink className="w-4 h-4" />
                      </Button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </Card>
      </div>

      {/* Bottom Row */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Network Reach */}
        <Card className="cyber-card">
          <div className="space-y-4">
            <h3 className="text-lg font-semibold">Network Reach</h3>
            
            <div className="space-y-4">
              {networkData.map((item, index) => (
                <div key={index} className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>{item.country}</span>
                    <span>{item.listeners} listeners</span>
                  </div>
                  <Progress value={item.percentage} className="h-2" />
                </div>
              ))}
            </div>
          </div>
        </Card>

        {/* Smart Contract Activity */}
        <Card className="cyber-card">
          <div className="space-y-4">
            <h3 className="text-lg font-semibold">Smart Contract Activity</h3>
            
            <div className="space-y-4">
              <div className="flex items-center justify-between p-3 rounded-lg bg-muted/30">
                <div>
                  <p className="text-sm font-medium">Last Mint Transaction</p>
                  <p className="text-xs text-muted-foreground">0xa1b2c3d4...ef56</p>
                </div>
                <Badge className="bg-green-500/20 text-green-400">Success</Badge>
              </div>

              <div className="flex items-center justify-between p-3 rounded-lg bg-muted/30">
                <div>
                  <p className="text-sm font-medium">IPFS Metadata</p>
                  <p className="text-xs text-muted-foreground">QmX1b2c3d4...hash</p>
                </div>
                <Badge className="bg-blue-500/20 text-blue-400">Pinned</Badge>
              </div>

              <div className="flex items-center justify-between p-3 rounded-lg bg-muted/30">
                <div>
                  <p className="text-sm font-medium">Active DAO Proposals</p>
                  <p className="text-xs text-muted-foreground">Story Collective #3</p>
                </div>
                <Badge className="bg-purple-500/20 text-purple-400">Voting</Badge>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
};

export default Dashboard;
