
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Wallet, Shield, Bell, User, Key, Coins, Settings as SettingsIcon } from "lucide-react";

const Settings = () => {
  const walletInfo = {
    address: "0x1234567890abcdef1234567890abcdef12345678",
    balance: "45.6 VOICE",
    usdValue: "$821.60",
    connected: true
  };

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Settings
          </h1>
          <p className="text-muted-foreground mt-2">
            Manage your account, wallet, and preferences
          </p>
        </div>
        
        <Badge 
          variant={walletInfo.connected ? "default" : "outline"} 
          className={walletInfo.connected ? "bg-green-500/20 text-green-400 border-green-500/30" : ""}
        >
          {walletInfo.connected ? "Wallet Connected" : "Wallet Disconnected"}
        </Badge>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Wallet Settings */}
        <Card className="cyber-card lg:col-span-2">
          <div className="space-y-6">
            <div className="flex items-center space-x-3">
              <div className="p-2 bg-primary/20 rounded-lg">
                <Wallet className="w-5 h-5 text-primary" />
              </div>
              <h2 className="text-xl font-semibold">Wallet & Payments</h2>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="p-4 bg-muted/20 rounded-lg">
                <h3 className="font-medium mb-2">Connected Wallet</h3>
                <p className="text-sm text-muted-foreground font-mono break-all">
                  {walletInfo.address}
                </p>
                <div className="flex items-center justify-between mt-3">
                  <div>
                    <p className="font-semibold text-primary">{walletInfo.balance}</p>
                    <p className="text-xs text-muted-foreground">{walletInfo.usdValue}</p>
                  </div>
                  <Button variant="outline" size="sm" className="border-primary/30">
                    Disconnect
                  </Button>
                </div>
              </div>
              
              <div className="p-4 bg-muted/20 rounded-lg">
                <h3 className="font-medium mb-2">Payment Preferences</h3>
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <Label htmlFor="auto-withdraw" className="text-sm">Auto-withdraw earnings</Label>
                    <Switch id="auto-withdraw" />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label htmlFor="gas-optimization" className="text-sm">Gas optimization</Label>
                    <Switch id="gas-optimization" defaultChecked />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Card>

        {/* Quick Actions */}
        <Card className="cyber-card">
          <div className="space-y-4">
            <h3 className="font-semibold">Quick Actions</h3>
            <div className="space-y-2">
              <Button variant="outline" className="w-full justify-start border-primary/30">
                <Key className="w-4 h-4 mr-2" />
                Export Private Key
              </Button>
              <Button variant="outline" className="w-full justify-start border-primary/30">
                <Coins className="w-4 h-4 mr-2" />
                View Transaction History
              </Button>
              <Button variant="outline" className="w-full justify-start border-primary/30">
                <Shield className="w-4 h-4 mr-2" />
                Security Settings
              </Button>
            </div>
          </div>
        </Card>
      </div>

      {/* Profile Settings */}
      <Card className="cyber-card">
        <div className="space-y-6">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-accent/20 rounded-lg">
              <User className="w-5 h-5 text-accent" />
            </div>
            <h2 className="text-xl font-semibold">Profile Settings</h2>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div>
                <Label htmlFor="display-name">Display Name</Label>
                <Input 
                  id="display-name" 
                  placeholder="Voice Creator"
                  className="bg-card/50 border-primary/20"
                />
              </div>
              <div>
                <Label htmlFor="bio">Bio</Label>
                <Input 
                  id="bio" 
                  placeholder="Professional voice artist and content creator"
                  className="bg-card/50 border-primary/20"
                />
              </div>
              <div>
                <Label htmlFor="website">Website</Label>
                <Input 
                  id="website" 
                  placeholder="https://yourwebsite.com"
                  className="bg-card/50 border-primary/20"
                />
              </div>
            </div>
            
            <div className="space-y-4">
              <div>
                <Label htmlFor="ens-name">ENS Name</Label>
                <Input 
                  id="ens-name" 
                  placeholder="yourname.eth"
                  className="bg-card/50 border-primary/20"
                />
              </div>
              <div>
                <Label htmlFor="lens-handle">Lens Handle</Label>
                <Input 
                  id="lens-handle" 
                  placeholder="@yourhandle.lens"
                  className="bg-card/50 border-primary/20"
                />
              </div>
              <div>
                <Label htmlFor="location">Location</Label>
                <Select>
                  <SelectTrigger className="bg-card/50 border-primary/20">
                    <SelectValue placeholder="Select your location" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="us">United States</SelectItem>
                    <SelectItem value="uk">United Kingdom</SelectItem>
                    <SelectItem value="ca">Canada</SelectItem>
                    <SelectItem value="au">Australia</SelectItem>
                    <SelectItem value="de">Germany</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Privacy & Security */}
      <Card className="cyber-card">
        <div className="space-y-6">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-orange-500/20 rounded-lg">
              <Shield className="w-5 h-5 text-orange-400" />
            </div>
            <h2 className="text-xl font-semibold">Privacy & Security</h2>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <h3 className="font-medium">AI Protection</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <Label htmlFor="ai-fingerprint" className="text-sm">Enable AI fingerprinting</Label>
                  <Switch id="ai-fingerprint" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="watermark" className="text-sm">Audio watermarking</Label>
                  <Switch id="watermark" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="clone-protection" className="text-sm">Clone protection</Label>
                  <Switch id="clone-protection" defaultChecked />
                </div>
              </div>
            </div>
            
            <div className="space-y-4">
              <h3 className="font-medium">Data Privacy</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <Label htmlFor="analytics-sharing" className="text-sm">Share analytics data</Label>
                  <Switch id="analytics-sharing" />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="public-profile" className="text-sm">Public profile</Label>
                  <Switch id="public-profile" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="activity-visibility" className="text-sm">Activity visibility</Label>
                  <Switch id="activity-visibility" defaultChecked />
                </div>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Notifications */}
      <Card className="cyber-card">
        <div className="space-y-6">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-blue-500/20 rounded-lg">
              <Bell className="w-5 h-5 text-blue-400" />
            </div>
            <h2 className="text-xl font-semibold">Notification Preferences</h2>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <h3 className="font-medium">NFT & Marketplace</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <Label htmlFor="nft-sales" className="text-sm">NFT sales notifications</Label>
                  <Switch id="nft-sales" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="bids" className="text-sm">New bids on your NFTs</Label>
                  <Switch id="bids" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="price-drops" className="text-sm">Price drop alerts</Label>
                  <Switch id="price-drops" />
                </div>
              </div>
            </div>
            
            <div className="space-y-4">
              <h3 className="font-medium">DAO & Governance</h3>
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <Label htmlFor="dao-proposals" className="text-sm">New DAO proposals</Label>
                  <Switch id="dao-proposals" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="voting-reminders" className="text-sm">Voting reminders</Label>
                  <Switch id="voting-reminders" defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="governance-updates" className="text-sm">Governance updates</Label>
                  <Switch id="governance-updates" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Save Changes */}
      <div className="flex justify-end space-x-4">
        <Button variant="outline" className="border-primary/30">
          Reset to Defaults
        </Button>
        <Button className="token-button">
          <SettingsIcon className="w-4 h-4 mr-2" />
          Save Changes
        </Button>
      </div>
    </div>
  );
};

export default Settings;
