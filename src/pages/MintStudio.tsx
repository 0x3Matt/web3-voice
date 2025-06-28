
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Badge } from "@/components/ui/badge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Mic, Upload, DollarSign, Globe, Lock } from "lucide-react";

const MintStudio = () => {
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
          Mint Studio
        </h1>
        <p className="text-muted-foreground mt-2">
          Transform your voice into valuable NFTs
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Mint Form */}
        <div className="lg:col-span-2 space-y-6">
          <Card className="cyber-card">
            <div className="space-y-6">
              <h2 className="text-xl font-semibold">Asset Selection</h2>
              
              {/* Selected File Preview */}
              <div className="p-4 bg-muted/30 rounded-lg border-2 border-dashed border-primary/30">
                <div className="flex items-center space-x-4">
                  <div className="w-12 h-12 bg-primary/20 rounded-lg flex items-center justify-center">
                    <Mic className="w-6 h-6 text-primary" />
                  </div>
                  <div>
                    <h3 className="font-medium">Morning Meditation Guide</h3>
                    <p className="text-sm text-muted-foreground">3:24 • 2.3 MB • WAV</p>
                  </div>
                  <Button variant="outline" className="ml-auto">
                    <Upload className="w-4 h-4 mr-2" />
                    Change File
                  </Button>
                </div>
              </div>
            </div>
          </Card>

          <Card className="cyber-card">
            <div className="space-y-6">
              <h2 className="text-xl font-semibold">NFT Metadata</h2>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-2">Title</label>
                  <Input placeholder="Enter NFT title" className="bg-muted/30" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-2">Category</label>
                  <Select>
                    <SelectTrigger className="bg-muted/30">
                      <SelectValue placeholder="Select category" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="meditation">Meditation</SelectItem>
                      <SelectItem value="music">Music</SelectItem>
                      <SelectItem value="story">Story</SelectItem>
                      <SelectItem value="education">Education</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Description</label>
                <Textarea 
                  placeholder="Describe your voice NFT..."
                  className="bg-muted/30 min-h-[100px]"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Tags</label>
                <Input placeholder="meditation, wellness, mindfulness" className="bg-muted/30" />
              </div>
            </div>
          </Card>

          <Card className="cyber-card">
            <div className="space-y-6">
              <h2 className="text-xl font-semibold">Access Control</h2>
              
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card className="p-4 border-2 border-primary/30 bg-primary/5">
                  <div className="text-center space-y-2">
                    <Globe className="w-8 h-8 text-primary mx-auto" />
                    <h3 className="font-medium">Public</h3>
                    <p className="text-xs text-muted-foreground">Anyone can purchase</p>
                  </div>
                </Card>
                
                <Card className="p-4 border-2 border-muted hover:border-accent/30 cursor-pointer">
                  <div className="text-center space-y-2">
                    <Lock className="w-8 h-8 text-muted-foreground mx-auto" />
                    <h3 className="font-medium">Token-Gated</h3>
                    <p className="text-xs text-muted-foreground">Requires specific tokens</p>
                  </div>
                </Card>
                
                <Card className="p-4 border-2 border-muted hover:border-accent/30 cursor-pointer">
                  <div className="text-center space-y-2">
                    <Mic className="w-8 h-8 text-muted-foreground mx-auto" />
                    <h3 className="font-medium">DAO-Only</h3>
                    <p className="text-xs text-muted-foreground">Community exclusive</p>
                  </div>
                </Card>
              </div>
            </div>
          </Card>
        </div>

        {/* Preview & Mint */}
        <div className="space-y-6">
          <Card className="cyber-card">
            <div className="space-y-4">
              <h2 className="text-xl font-semibold">NFT Preview</h2>
              
              <div className="aspect-square bg-gradient-to-br from-primary/20 to-accent/20 rounded-xl p-6 flex flex-col items-center justify-center border border-primary/30">
                <Mic className="w-16 h-16 text-primary mb-4" />
                <h3 className="font-bold text-lg text-center">Morning Meditation Guide</h3>
                <p className="text-sm text-muted-foreground text-center">Voice NFT • 3:24</p>
                <Badge className="mt-2 bg-primary/20 text-primary">Meditation</Badge>
              </div>

              <div className="space-y-3">
                <div className="flex justify-between text-sm">
                  <span>Creator</span>
                  <span className="text-primary">Voice Creator Pro</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span>Chain</span>
                  <span>Ethereum</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span>Royalty</span>
                  <span>10%</span>
                </div>
              </div>
            </div>
          </Card>

          <Card className="cyber-card">
            <div className="space-y-4">
              <h2 className="text-xl font-semibold">Mint Settings</h2>
              
              <div>
                <label className="block text-sm font-medium mb-2">Blockchain</label>
                <Select>
                  <SelectTrigger className="bg-muted/30">
                    <SelectValue placeholder="Ethereum" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="ethereum">Ethereum</SelectItem>
                    <SelectItem value="polygon">Polygon</SelectItem>
                    <SelectItem value="arbitrum">Arbitrum</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">Initial Price</label>
                <div className="flex">
                  <Input placeholder="0.1" className="bg-muted/30" />
                  <Select>
                    <SelectTrigger className="w-20 bg-muted/30 ml-2">
                      <SelectValue placeholder="ETH" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="eth">ETH</SelectItem>
                      <SelectItem value="voice">VOICE</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div className="p-4 bg-muted/30 rounded-lg space-y-2">
                <div className="flex justify-between text-sm">
                  <span>Mint Cost</span>
                  <span>~$45</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span>Platform Fee</span>
                  <span>2.5%</span>
                </div>
                <div className="flex justify-between text-sm font-medium border-t border-border/50 pt-2">
                  <span>Total</span>
                  <span>~$47</span>
                </div>
              </div>

              <Button className="w-full token-button">
                <DollarSign className="w-4 h-4 mr-2" />
                Mint NFT
              </Button>
            </div>
          </Card>
        </div>
      </div>
    </div>
  );
};

export default MintStudio;
