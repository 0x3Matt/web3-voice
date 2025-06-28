
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Play, Heart, ShoppingCart, Search, Filter } from "lucide-react";

const Marketplace = () => {
  const nftListings = [
    {
      id: 1,
      title: "Midnight Jazz Session",
      creator: "JazzMaster.eth",
      price: "2.5 VOICE",
      usdPrice: "$45.00",
      duration: "4:23",
      category: "Music",
      verified: true,
      likes: 342,
      waveform: [0.3, 0.7, 0.5, 0.9, 0.4, 0.8, 0.6, 0.2, 0.9, 0.5]
    },
    {
      id: 2,
      title: "Sci-Fi Audiobook Chapter 1",
      creator: "VoiceArtist.eth",
      price: "1.8 VOICE",
      usdPrice: "$32.40",
      duration: "12:45",
      category: "Storytelling",
      verified: true,
      likes: 128,
      waveform: [0.5, 0.4, 0.8, 0.6, 0.9, 0.3, 0.7, 0.5, 0.8, 0.4]
    },
    {
      id: 3,
      title: "Meditation Guide: Inner Peace",
      creator: "ZenVoice.eth",
      price: "0.9 VOICE",
      usdPrice: "$16.20",
      duration: "8:30",
      category: "Wellness",
      verified: false,
      likes: 89,
      waveform: [0.2, 0.6, 0.4, 0.7, 0.3, 0.8, 0.5, 0.9, 0.4, 0.6]
    },
    {
      id: 4,
      title: "Crypto Analysis Deep Dive",
      creator: "CryptoGuru.eth",
      price: "3.2 VOICE",
      usdPrice: "$57.60",
      duration: "15:12",
      category: "Education",
      verified: true,
      likes: 256,
      waveform: [0.7, 0.5, 0.9, 0.4, 0.8, 0.6, 0.3, 0.9, 0.5, 0.7]
    },
    {
      id: 5,
      title: "Original Poetry Reading",
      creator: "Poet.eth",
      price: "1.2 VOICE",
      usdPrice: "$21.60",
      duration: "6:18",
      category: "Literature",
      verified: false,
      likes: 67,
      waveform: [0.4, 0.8, 0.3, 0.9, 0.5, 0.7, 0.4, 0.8, 0.6, 0.3]
    },
    {
      id: 6,
      title: "Gaming Commentary Highlight",
      creator: "GameVoice.eth",
      price: "2.1 VOICE",
      usdPrice: "$37.80",
      duration: "9:45",
      category: "Gaming",
      verified: true,
      likes: 194,
      waveform: [0.6, 0.9, 0.4, 0.7, 0.8, 0.3, 0.9, 0.5, 0.6, 0.8]
    }
  ];

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            Voice Marketplace
          </h1>
          <p className="text-muted-foreground mt-2">
            Discover and collect authentic voice NFTs
          </p>
        </div>
        
        <div className="flex items-center space-x-4">
          <Badge variant="outline" className="border-primary/30 text-primary">
            {nftListings.length} NFTs Available
          </Badge>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="flex flex-col md:flex-row gap-4 items-start md:items-center">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground w-4 h-4" />
          <Input 
            placeholder="Search voice NFTs..." 
            className="pl-10 bg-card/50 border-primary/20"
          />
        </div>
        
        <div className="flex items-center space-x-4">
          <Select defaultValue="all">
            <SelectTrigger className="w-[180px] bg-card/50 border-primary/20">
              <SelectValue placeholder="Category" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Categories</SelectItem>
              <SelectItem value="music">Music</SelectItem>
              <SelectItem value="storytelling">Storytelling</SelectItem>
              <SelectItem value="education">Education</SelectItem>
              <SelectItem value="wellness">Wellness</SelectItem>
              <SelectItem value="gaming">Gaming</SelectItem>
            </SelectContent>
          </Select>
          
          <Select defaultValue="recent">
            <SelectTrigger className="w-[150px] bg-card/50 border-primary/20">
              <SelectValue placeholder="Sort by" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="recent">Most Recent</SelectItem>
              <SelectItem value="price-low">Price: Low to High</SelectItem>
              <SelectItem value="price-high">Price: High to Low</SelectItem>
              <SelectItem value="popular">Most Popular</SelectItem>
            </SelectContent>
          </Select>
          
          <Button variant="outline" size="icon" className="border-primary/30">
            <Filter className="w-4 h-4" />
          </Button>
        </div>
      </div>

      {/* NFT Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {nftListings.map((nft) => (
          <Card key={nft.id} className="cyber-card group hover:scale-105 transition-all duration-300">
            <div className="space-y-4">
              {/* Waveform Visualization */}
              <div className="flex items-center justify-center space-x-1 h-20 bg-muted/30 rounded-lg p-4 relative">
                {nft.waveform.map((height, i) => (
                  <div
                    key={i}
                    className="w-2 bg-primary/60 rounded-full waveform-pulse"
                    style={{ 
                      height: `${height * 50}px`,
                      animationDelay: `${i * 0.1}s`
                    }}
                  />
                ))}
                <Button 
                  size="sm" 
                  className="absolute bg-primary/20 text-primary hover:bg-primary/30 rounded-full"
                >
                  <Play className="w-4 h-4" />
                </Button>
              </div>

              <div className="space-y-3">
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="font-semibold text-lg mb-1">{nft.title}</h3>
                    <div className="flex items-center space-x-2">
                      <span className="text-sm text-muted-foreground">by {nft.creator}</span>
                      {nft.verified && (
                        <Badge variant="secondary" className="text-xs bg-primary/20 text-primary">
                          Verified
                        </Badge>
                      )}
                    </div>
                  </div>
                  <Button variant="ghost" size="sm">
                    <Heart className="w-4 h-4" />
                  </Button>
                </div>
                
                <div className="flex items-center justify-between text-sm text-muted-foreground">
                  <span>{nft.duration}</span>
                  <div className="flex items-center space-x-1">
                    <Heart className="w-3 h-3" />
                    <span>{nft.likes}</span>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <Badge variant="outline" className="text-xs">
                    {nft.category}
                  </Badge>
                  <div className="text-right">
                    <div className="font-bold text-primary">{nft.price}</div>
                    <div className="text-xs text-muted-foreground">{nft.usdPrice}</div>
                  </div>
                </div>
              </div>

              {/* Action Buttons */}
              <div className="flex space-x-2 pt-4 border-t border-border/50">
                <Button className="flex-1 token-button">
                  <ShoppingCart className="w-4 h-4 mr-2" />
                  Buy Now
                </Button>
                <Button variant="outline" className="border-primary/30">
                  View Details
                </Button>
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default Marketplace;
