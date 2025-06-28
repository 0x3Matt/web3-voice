
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Wallet, Shield, Bell, Globe, User, Key, Download, Upload } from "lucide-react";

const Settings = () => {
  return (
    <div className="space-y-4 sm:space-y-6 lg:space-y-8 p-4 sm:p-6 lg:p-8">
      <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
        <div>
          <h1 className="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Settings
          </h1>
          <p className="text-muted-foreground mt-1 sm:mt-2 text-sm sm:text-base">
            Manage your account and platform preferences
          </p>
        </div>
        
        <div className="flex flex-col sm:flex-row gap-2 sm:gap-4">
          <Button variant="outline" className="border-primary/30 text-xs sm:text-sm">
            <Download className="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
            Export Data
          </Button>
          <Button className="token-button text-xs sm:text-sm">
            <Upload className="w-3 h-3 sm:w-4 sm:h-4 mr-2" />
            Import Settings
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6 lg:gap-8">
        {/* Main Settings Column */}
        <div className="lg:col-span-2 space-y-4 sm:space-y-6">
          {/* Wallet & Identity */}
          <Card className="cyber-card">
            <div className="space-y-4 sm:space-y-6">
              <div className="flex items-center space-x-3">
                <div className="p-2 sm:p-3 bg-primary/20 rounded-lg glow-primary">
                  <Wallet className="w-4 h-4 sm:w-6 sm:h-6 text-primary" />
                </div>
                <h2 className="text-lg sm:text-xl font-semibold">Wallet & Identity</h2>
              </div>
              
              <div className="space-y-4">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-3 sm:p-4 bg-muted/20 rounded-lg">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">Connected Wallet</p>
                    <p className="text-xs sm:text-sm text-muted-foreground font-mono">0x1234...5678</p>
                  </div>
                  <div className="flex flex-col sm:flex-row gap-2">
                    <Badge className="bg-green-500/20 text-green-400 text-xs">Connected</Badge>
                    <Button size="sm" variant="outline" className="text-xs">
                      Disconnect
                    </Button>
                  </div>
                </div>
                
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <div>
                    <Label className="text-xs sm:text-sm">ENS Name</Label>
                    <Input placeholder="yourname.eth" className="bg-muted/30 text-xs sm:text-sm" />
                  </div>
                  <div>
                    <Label className="text-xs sm:text-sm">Lens Handle</Label>
                    <Input placeholder="@yourhandle" className="bg-muted/30 text-xs sm:text-sm" />
                  </div>
                </div>
              </div>
            </div>
          </Card>

          {/* Security Settings */}
          <Card className="cyber-card">
            <div className="space-y-4 sm:space-y-6">
              <div className="flex items-center space-x-3">
                <div className="p-2 sm:p-3 bg-accent/20 rounded-lg">
                  <Shield className="w-4 h-4 sm:w-6 sm:h-6 text-accent" />
                </div>
                <h2 className="text-lg sm:text-xl font-semibold">Security</h2>
              </div>
              
              <div className="space-y-4 sm:space-y-6">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">Two-Factor Authentication</p>
                    <p className="text-xs sm:text-sm text-muted-foreground">Add an extra layer of security</p>
                  </div>
                  <Switch />
                </div>
                
                <Separator />
                
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">AI Clone Protection</p>
                    <p className="text-xs sm:text-sm text-muted-foreground">Prevent unauthorized voice cloning</p>
                  </div>
                  <Switch defaultChecked />
                </div>
                
                <Separator />
                
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">Watermark Protection</p>
                    <p className="text-xs sm:text-sm text-muted-foreground">Add invisible watermarks to uploads</p>
                  </div>
                  <Switch defaultChecked />
                </div>
              </div>
            </div>
          </Card>

          {/* Privacy & Permissions */}
          <Card className="cyber-card">
            <div className="space-y-4 sm:space-y-6">
              <div className="flex items-center space-x-3">
                <div className="p-2 sm:p-3 bg-blue-500/20 rounded-lg">
                  <User className="w-4 h-4 sm:w-6 sm:h-6 text-blue-400" />
                </div>
                <h2 className="text-lg sm:text-xl font-semibold">Privacy & Permissions</h2>
              </div>
              
              <div className="space-y-4 sm:space-y-6">
                <div>
                  <Label className="text-xs sm:text-sm mb-2 block">Profile Visibility</Label>
                  <Select>
                    <SelectTrigger className="bg-muted/30">
                      <SelectValue placeholder="Public" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="public">Public</SelectItem>
                      <SelectItem value="friends">Friends Only</SelectItem>
                      <SelectItem value="private">Private</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">Analytics Tracking</p>
                    <p className="text-xs sm:text-sm text-muted-foreground">Allow usage analytics collection</p>
                  </div>
                  <Switch defaultChecked />
                </div>
                
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div className="space-y-1">
                    <p className="font-medium text-sm sm:text-base">Marketing Communications</p>
                    <p className="text-xs sm:text-sm text-muted-foreground">Receive updates and promotions</p>
                  </div>
                  <Switch />
                </div>
              </div>
            </div>
          </Card>
        </div>

        {/* Sidebar Settings */}
        <div className="space-y-4 sm:space-y-6">
          {/* Notifications */}
          <Card className="cyber-card">
            <div className="space-y-4">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-orange-500/20 rounded-lg">
                  <Bell className="w-4 h-4 text-orange-400" />
                </div>
                <h3 className="text-base sm:text-lg font-semibold">Notifications</h3>
              </div>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-xs sm:text-sm">New Purchases</span>
                  <Switch defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-xs sm:text-sm">DAO Votes</span>
                  <Switch defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-xs sm:text-sm">Royalty Payments</span>
                  <Switch defaultChecked />
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-xs sm:text-sm">System Updates</span>
                  <Switch />
                </div>
              </div>
            </div>
          </Card>

          {/* Language & Region */}
          <Card className="cyber-card">
            <div className="space-y-4">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-green-500/20 rounded-lg">
                  <Globe className="w-4 h-4 text-green-400" />
                </div>
                <h3 className="text-base sm:text-lg font-semibold">Language & Region</h3>
              </div>
              
              <div className="space-y-4">
                <div>
                  <Label className="text-xs sm:text-sm mb-2 block">Language</Label>
                  <Select>
                    <SelectTrigger className="bg-muted/30">
                      <SelectValue placeholder="English" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="en">English</SelectItem>
                      <SelectItem value="es">Spanish</SelectItem>
                      <SelectItem value="fr">French</SelectItem>
                      <SelectItem value="de">German</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div>
                  <Label className="text-xs sm:text-sm mb-2 block">Currency</Label>
                  <Select>
                    <SelectTrigger className="bg-muted/30">
                      <SelectValue placeholder="USD" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="usd">USD</SelectItem>
                      <SelectItem value="eur">EUR</SelectItem>
                      <SelectItem value="eth">ETH</SelectItem>
                      <SelectItem value="voice">VOICE</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>
          </Card>

          {/* API Keys */}
          <Card className="cyber-card">
            <div className="space-y-4">
              <div className="flex items-center space-x-3">
                <div className="p-2 bg-purple-500/20 rounded-lg">
                  <Key className="w-4 h-4 text-purple-400" />
                </div>
                <h3 className="text-base sm:text-lg font-semibold">API Access</h3>
              </div>
              
              <div className="space-y-3">
                <div className="p-3 bg-muted/20 rounded-lg">
                  <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                    <div>
                      <p className="font-medium text-xs sm:text-sm">Production Key</p>
                      <p className="text-xs text-muted-foreground font-mono">vw3_prod_***</p>
                    </div>
                    <Button size="sm" variant="outline" className="text-xs">
                      Regenerate
                    </Button>
                  </div>
                </div>
                
                <Button size="sm" className="w-full bg-primary/20 text-primary hover:bg-primary/30 text-xs">
                  Generate New Key
                </Button>
              </div>
            </div>
          </Card>
        </div>
      </div>

      {/* Save Changes Button */}
      <div className="flex flex-col sm:flex-row gap-4 pt-4 sm:pt-6 border-t border-border/50">
        <Button className="token-button flex-1 sm:flex-none">
          Save Changes
        </Button>
        <Button variant="outline" className="border-muted flex-1 sm:flex-none">
          Reset to Defaults
        </Button>
      </div>
    </div>
  );
};

export default Settings;
