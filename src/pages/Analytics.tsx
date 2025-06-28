
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { BarChart, TrendingUp, Users, Coins, Play, Download } from "lucide-react";

const Analytics = () => {
  const analyticsData = {
    totalPlays: 12847,
    totalEarnings: 45.6,
    uniqueListeners: 3421,
    avgListenTime: "4:32",
    topRegions: ["United States", "United Kingdom", "Germany", "Canada", "Australia"],
    recentActivity: [
      { action: "NFT Purchase", user: "0x1234...5678", amount: "2.5 VOICE", time: "2 hours ago" },
      { action: "Voice Play", user: "0xabcd...efgh", amount: "+0.1 VOICE", time: "3 hours ago" },
      { action: "DAO Vote", user: "0x9876...5432", amount: "Governance", time: "5 hours ago" },
      { action: "Royalty Payment", user: "System", amount: "+1.2 VOICE", time: "6 hours ago" }
    ]
  };

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Analytics Dashboard
          </h1>
          <p className="text-muted-foreground mt-2">
            Track your voice content performance and earnings
          </p>
        </div>
        
        <div className="flex items-center space-x-4">
          <Select defaultValue="30d">
            <SelectTrigger className="w-[150px] bg-card/50 border-primary/20">
              <SelectValue placeholder="Time Range" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="7d">Last 7 days</SelectItem>
              <SelectItem value="30d">Last 30 days</SelectItem>
              <SelectItem value="90d">Last 90 days</SelectItem>
              <SelectItem value="1y">Last year</SelectItem>
            </SelectContent>
          </Select>
          <Button variant="outline" className="border-primary/30">
            <Download className="w-4 h-4 mr-2" />
            Export
          </Button>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card className="cyber-card">
          <div className="flex items-center space-x-4">
            <div className="p-3 bg-primary/20 rounded-lg glow-primary">
              <Play className="w-6 h-6 text-primary" />
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Total Plays</p>
              <p className="text-2xl font-bold">{analyticsData.totalPlays.toLocaleString()}</p>
              <div className="flex items-center space-x-1 text-green-400 text-xs">
                <TrendingUp className="w-3 h-3" />
                <span>+12.5%</span>
              </div>
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center space-x-4">
            <div className="p-3 bg-accent/20 rounded-lg">
              <Coins className="w-6 h-6 text-accent" />
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Total Earnings</p>
              <p className="text-2xl font-bold">{analyticsData.totalEarnings} VOICE</p>
              <div className="flex items-center space-x-1 text-green-400 text-xs">
                <TrendingUp className="w-3 h-3" />
                <span>+8.3%</span>
              </div>
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center space-x-4">
            <div className="p-3 bg-blue-500/20 rounded-lg">
              <Users className="w-6 h-6 text-blue-400" />
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Unique Listeners</p>
              <p className="text-2xl font-bold">{analyticsData.uniqueListeners.toLocaleString()}</p>
              <div className="flex items-center space-x-1 text-green-400 text-xs">
                <TrendingUp className="w-3 h-3" />
                <span>+15.7%</span>
              </div>
            </div>
          </div>
        </Card>

        <Card className="cyber-card">
          <div className="flex items-center space-x-4">
            <div className="p-3 bg-orange-500/20 rounded-lg">
              <BarChart className="w-6 h-6 text-orange-400" />
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Avg Listen Time</p>
              <p className="text-2xl font-bold">{analyticsData.avgListenTime}</p>
              <div className="flex items-center space-x-1 text-green-400 text-xs">
                <TrendingUp className="w-3 h-3" />
                <span>+5.2%</span>
              </div>
            </div>
          </div>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Geographic Distribution */}
        <Card className="cyber-card">
          <div className="space-y-4">
            <h3 className="text-xl font-semibold">Geographic Distribution</h3>
            <div className="space-y-3">
              {analyticsData.topRegions.map((region, index) => (
                <div key={region} className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-primary/20 rounded-full flex items-center justify-center text-primary font-semibold text-sm">
                      {index + 1}
                    </div>
                    <span>{region}</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <div className="w-20 h-2 bg-muted rounded-full overflow-hidden">
                      <div 
                        className="h-full bg-primary rounded-full"
                        style={{ width: `${(5 - index) * 20}%` }}
                      />
                    </div>
                    <span className="text-sm text-muted-foreground">{(5 - index) * 20}%</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </Card>

        {/* Recent Activity */}
        <Card className="cyber-card">
          <div className="space-y-4">
            <h3 className="text-xl font-semibold">Recent Activity</h3>
            <div className="space-y-4">
              {analyticsData.recentActivity.map((activity, index) => (
                <div key={index} className="flex items-center justify-between p-3 bg-muted/20 rounded-lg">
                  <div>
                    <p className="font-medium">{activity.action}</p>
                    <p className="text-sm text-muted-foreground">{activity.user}</p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium text-primary">{activity.amount}</p>
                    <p className="text-xs text-muted-foreground">{activity.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </Card>
      </div>

      {/* Performance Chart Placeholder */}
      <Card className="cyber-card">
        <div className="space-y-4">
          <h3 className="text-xl font-semibold">Earnings Over Time</h3>
          <div className="h-64 bg-muted/20 rounded-lg flex items-center justify-center">
            <div className="text-center text-muted-foreground">
              <BarChart className="w-12 h-12 mx-auto mb-2 opacity-50" />
              <p>Chart visualization would appear here</p>
              <p className="text-sm">Integration with charting library needed</p>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default Analytics;
