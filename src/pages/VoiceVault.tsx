
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Upload, Mic, Play, Edit, Share, Coins } from "lucide-react";

const VoiceVault = () => {
  const voiceFiles = [
    {
      id: 1,
      title: "Morning Meditation Guide",
      category: "Minted",
      duration: "3:24",
      size: "2.3 MB",
      protection: "AI Protected",
      waveform: [0.2, 0.8, 0.6, 0.9, 0.4, 0.7, 0.3, 0.8, 0.5, 0.9]
    },
    {
      id: 2,
      title: "Crypto Analysis Deep Dive",
      category: "Draft",
      duration: "12:45",
      size: "8.1 MB",
      protection: "Watermarked",
      waveform: [0.4, 0.6, 0.8, 0.5, 0.9, 0.3, 0.7, 0.6, 0.8, 0.4]
    },
    {
      id: 3,
      title: "Story Chapter 2",
      category: "DAO Submission",
      duration: "8:12",
      size: "5.4 MB",
      protection: "Encrypted",
      waveform: [0.6, 0.4, 0.9, 0.7, 0.3, 0.8, 0.5, 0.9, 0.2, 0.7]
    }
  ];

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Voice Vault
          </h1>
          <p className="text-muted-foreground mt-2">
            Manage and organize your voice assets
          </p>
        </div>
        
        <div className="flex space-x-4">
          <Button variant="outline" className="border-primary/30 hover:bg-primary/10">
            <Mic className="w-4 h-4 mr-2" />
            Record New
          </Button>
          <Button className="token-button">
            <Upload className="w-4 h-4 mr-2" />
            Upload File
          </Button>
        </div>
      </div>

      {/* Category Filters */}
      <div className="flex space-x-4">
        {["All", "Draft", "Minted", "Archived", "DAO Submissions"].map((category) => (
          <Button
            key={category}
            variant={category === "All" ? "default" : "outline"}
            className={category === "All" ? "bg-primary/20 text-primary border-primary/30" : "border-muted"}
          >
            {category}
          </Button>
        ))}
      </div>

      {/* Voice Files Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {voiceFiles.map((file) => (
          <Card key={file.id} className="cyber-card group hover:scale-105 transition-transform duration-300">
            <div className="space-y-4">
              {/* Waveform Visualization */}
              <div className="flex items-center justify-center space-x-1 h-16 bg-muted/30 rounded-lg p-4">
                {file.waveform.map((height, i) => (
                  <div
                    key={i}
                    className="w-2 bg-primary/60 rounded-full waveform-pulse"
                    style={{ 
                      height: `${height * 40}px`,
                      animationDelay: `${i * 0.1}s`
                    }}
                  />
                ))}
              </div>

              <div>
                <h3 className="font-semibold text-lg mb-2">{file.title}</h3>
                <div className="flex items-center justify-between text-sm text-muted-foreground mb-3">
                  <span>{file.duration}</span>
                  <span>{file.size}</span>
                </div>
                
                <div className="flex items-center justify-between mb-4">
                  <Badge 
                    variant={file.category === "Minted" ? "default" : "outline"}
                    className={file.category === "Minted" ? "bg-primary/20 text-primary" : ""}
                  >
                    {file.category}
                  </Badge>
                  <Badge variant="secondary" className="text-xs">
                    {file.protection}
                  </Badge>
                </div>
              </div>

              {/* Action Buttons */}
              <div className="flex items-center justify-between pt-4 border-t border-border/50">
                <div className="flex space-x-2">
                  <Button size="sm" variant="ghost">
                    <Play className="w-4 h-4" />
                  </Button>
                  <Button size="sm" variant="ghost">
                    <Edit className="w-4 h-4" />
                  </Button>
                  <Button size="sm" variant="ghost">
                    <Share className="w-4 h-4" />
                  </Button>
                </div>
                
                {file.category === "Draft" && (
                  <Button size="sm" className="bg-primary/20 text-primary hover:bg-primary/30">
                    <Coins className="w-4 h-4 mr-1" />
                    Mint
                  </Button>
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default VoiceVault;
