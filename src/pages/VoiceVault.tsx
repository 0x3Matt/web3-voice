
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Upload, Mic, Play, Edit, Share, Coins, MoreVertical } from "lucide-react";

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
    <div className="space-y-4 sm:space-y-6 lg:space-y-8 max-w-full">
      {/* Header Section */}
      <div className="flex flex-col space-y-4 sm:flex-row sm:justify-between sm:items-start sm:space-y-0">
        <div className="min-w-0">
          <h1 className="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent leading-tight">
            Voice Vault
          </h1>
          <p className="text-muted-foreground mt-1 sm:mt-2 text-sm sm:text-base">
            Manage and organize your voice assets
          </p>
        </div>
        
        {/* Action Buttons */}
        <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-3 w-full sm:w-auto">
          <Button variant="outline" className="border-primary/30 hover:bg-primary/10 w-full sm:w-auto">
            <Mic className="w-4 h-4 mr-2" />
            <span className="sm:inline">Record New</span>
          </Button>
          <Button className="token-button w-full sm:w-auto">
            <Upload className="w-4 h-4 mr-2" />
            <span className="sm:inline">Upload File</span>
          </Button>
        </div>
      </div>

      {/* Category Filters - Horizontal scroll on mobile */}
      <div className="flex space-x-2 sm:space-x-4 overflow-x-auto pb-2 scrollbar-hide">
        {["All", "Draft", "Minted", "Archived", "DAO Submissions"].map((category) => (
          <Button
            key={category}
            variant={category === "All" ? "default" : "outline"}
            className={`whitespace-nowrap text-xs sm:text-sm px-3 sm:px-4 py-2 ${
              category === "All" 
                ? "bg-primary/20 text-primary border-primary/30" 
                : "border-muted"
            }`}
          >
            {category}
          </Button>
        ))}
      </div>

      {/* Voice Files Grid - Responsive columns */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
        {voiceFiles.map((file) => (
          <Card key={file.id} className="cyber-card group hover:scale-105 transition-transform duration-300 overflow-hidden">
            <div className="space-y-3 sm:space-y-4 p-4 sm:p-6">
              {/* Waveform Visualization - Responsive height */}
              <div className="flex items-center justify-center space-x-1 h-12 sm:h-16 bg-muted/30 rounded-lg p-3 sm:p-4">
                {file.waveform.map((height, i) => (
                  <div
                    key={i}
                    className="w-1.5 sm:w-2 bg-primary/60 rounded-full waveform-pulse"
                    style={{ 
                      height: `${height * 30}px`,
                      animationDelay: `${i * 0.1}s`
                    }}
                  />
                ))}
              </div>

              <div className="space-y-2 sm:space-y-3">
                <h3 className="font-semibold text-base sm:text-lg leading-tight line-clamp-2">{file.title}</h3>
                <div className="flex items-center justify-between text-xs sm:text-sm text-muted-foreground">
                  <span>{file.duration}</span>
                  <span>{file.size}</span>
                </div>
                
                <div className="flex items-center justify-between flex-wrap gap-2">
                  <Badge 
                    variant={file.category === "Minted" ? "default" : "outline"}
                    className={`text-xs ${file.category === "Minted" ? "bg-primary/20 text-primary" : ""}`}
                  >
                    {file.category}
                  </Badge>
                  <Badge variant="secondary" className="text-xs">
                    {file.protection}
                  </Badge>
                </div>
              </div>

              {/* Action Buttons - Mobile optimized */}
              <div className="flex items-center justify-between pt-3 sm:pt-4 border-t border-border/50">
                <div className="flex space-x-1 sm:space-x-2">
                  <Button size="sm" variant="ghost" className="h-8 w-8 sm:h-9 sm:w-9">
                    <Play className="w-3 h-3 sm:w-4 sm:h-4" />
                  </Button>
                  <Button size="sm" variant="ghost" className="h-8 w-8 sm:h-9 sm:w-9">
                    <Edit className="w-3 h-3 sm:w-4 sm:h-4" />
                  </Button>
                  <Button size="sm" variant="ghost" className="h-8 w-8 sm:h-9 sm:w-9">
                    <Share className="w-3 h-3 sm:w-4 sm:h-4" />
                  </Button>
                  {/* More actions on mobile */}
                  <Button size="sm" variant="ghost" className="h-8 w-8 sm:hidden">
                    <MoreVertical className="w-3 h-3" />
                  </Button>
                </div>
                
                {file.category === "Draft" && (
                  <Button size="sm" className="bg-primary/20 text-primary hover:bg-primary/30 text-xs sm:text-sm px-2 sm:px-3">
                    <Coins className="w-3 h-3 sm:w-4 sm:h-4 mr-1" />
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
