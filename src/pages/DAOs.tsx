
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Users, Vote, Coins, Clock, CheckCircle, XCircle } from "lucide-react";

const DAOs = () => {
  const daos = [
    {
      id: 1,
      name: "VoiceDAO Collective",
      description: "The premier DAO for voice content creators and collectors",
      members: 2847,
      totalValue: "125.4K VOICE",
      userRole: "Member",
      joined: true,
      logo: "🎙️"
    },
    {
      id: 2,
      name: "StoryChain DAO",
      description: "Collaborative storytelling and audiobook creation",
      members: 1203,
      totalValue: "67.8K VOICE",
      userRole: "Contributor",
      joined: true,
      logo: "📚"
    },
    {
      id: 3,
      name: "AudioNFT Investors",
      description: "Investment-focused DAO for premium voice NFTs",
      members: 456,
      totalValue: "340.2K VOICE",
      userRole: null,
      joined: false,
      logo: "💎"
    }
  ];

  const activeProposals = [
    {
      id: 1,
      title: "Increase Creator Royalty Split to 15%",
      description: "Proposal to increase the standard royalty split for voice creators from 10% to 15%",
      daoName: "VoiceDAO Collective",
      votesFor: 1247,
      votesAgainst: 356,
      totalVotes: 1603,
      timeLeft: "2 days",
      status: "active",
      userVoted: false
    },
    {
      id: 2,
      title: "Fund New Story Series: 'Crypto Chronicles'",
      description: "Allocate 50K VOICE tokens to fund a new collaborative story series",
      daoName: "StoryChain DAO",
      votesFor: 234,
      votesAgainst: 89,
      totalVotes: 323,
      timeLeft: "5 days",
      status: "active",
      userVoted: true
    },
    {
      id: 3,
      title: "Partnership with Major Podcast Platform",
      description: "Establish partnership agreement with leading podcast distribution platform",
      daoName: "VoiceDAO Collective",
      votesFor: 892,
      votesAgainst: 234,
      totalVotes: 1126,
      timeLeft: "Ended",
      status: "passed",
      userVoted: true
    }
  ];

  const userContributions = [
    {
      dao: "VoiceDAO Collective",
      type: "Voice Submission",
      title: "Meditation Series Part 3",
      reward: "12.5 VOICE",
      status: "Approved",
      date: "2 days ago"
    },
    {
      dao: "StoryChain DAO",
      type: "Chapter Narration",
      title: "Crypto Chronicles Ch. 1",
      reward: "25.0 VOICE",
      status: "In Review",
      date: "1 week ago"
    }
  ];

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
            DAOs & Collectives
          </h1>
          <p className="text-muted-foreground mt-2">
            Participate in decentralized voice content governance
          </p>
        </div>
        
        <Badge variant="outline" className="border-primary/30 text-primary">
          {daos.filter(dao => dao.joined).length} DAOs Joined
        </Badge>
      </div>

      {/* DAOs Overview */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {daos.map((dao) => (
          <Card key={dao.id} className="cyber-card">
            <div className="space-y-4">
              <div className="flex items-start justify-between">
                <div className="flex items-center space-x-3">
                  <div className="text-2xl">{dao.logo}</div>
                  <div>
                    <h3 className="font-semibold text-lg">{dao.name}</h3>
                    {dao.joined && (
                      <Badge variant="secondary" className="text-xs bg-primary/20 text-primary">
                        {dao.userRole}
                      </Badge>
                    )}
                  </div>
                </div>
              </div>
              
              <p className="text-sm text-muted-foreground">{dao.description}</p>
              
              <div className="flex items-center justify-between text-sm">
                <div className="flex items-center space-x-1">
                  <Users className="w-4 h-4 text-muted-foreground" />
                  <span>{dao.members.toLocaleString()} members</span>
                </div>
                <div className="flex items-center space-x-1">
                  <Coins className="w-4 h-4 text-primary" />
                  <span className="text-primary font-medium">{dao.totalValue}</span>
                </div>
              </div>
              
              <div className="pt-4 border-t border-border/50">
                {dao.joined ? (
                  <Button variant="outline" className="w-full border-primary/30">
                    View Dashboard
                  </Button>
                ) : (
                  <Button className="w-full token-button">
                    Join DAO
                  </Button>
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>

      {/* Active Proposals */}
      <Card className="cyber-card">
        <div className="space-y-6">
          <h2 className="text-2xl font-semibold">Active Proposals</h2>
          
          <div className="space-y-4">
            {activeProposals.map((proposal) => (
              <div key={proposal.id} className="p-4 bg-muted/20 rounded-lg border border-border/50">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-2">
                      <h3 className="font-semibold">{proposal.title}</h3>
                      {proposal.status === "passed" && (
                        <Badge className="bg-green-500/20 text-green-400 border-green-500/30">
                          <CheckCircle className="w-3 h-3 mr-1" />
                          Passed
                        </Badge>
                      )}
                      {proposal.status === "active" && (
                        <Badge variant="outline" className="border-orange-500/30 text-orange-400">
                          <Clock className="w-3 h-3 mr-1" />
                          Active
                        </Badge>
                      )}
                      {proposal.userVoted && (
                        <Badge variant="secondary" className="text-xs">
                          Voted
                        </Badge>
                      )}
                    </div>
                    <p className="text-sm text-muted-foreground mb-2">{proposal.description}</p>
                    <p className="text-xs text-muted-foreground">by {proposal.daoName}</p>
                  </div>
                  
                  <div className="text-right text-sm">
                    <p className="text-muted-foreground">{proposal.timeLeft}</p>
                  </div>
                </div>
                
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span>For: {proposal.votesFor}</span>
                    <span>Against: {proposal.votesAgainst}</span>
                  </div>
                  <Progress 
                    value={(proposal.votesFor / proposal.totalVotes) * 100} 
                    className="h-2"
                  />
                  <div className="flex items-center justify-between">
                    <span className="text-xs text-muted-foreground">
                      {proposal.totalVotes} total votes
                    </span>
                    {proposal.status === "active" && !proposal.userVoted && (
                      <div className="flex space-x-2">
                        <Button size="sm" variant="outline" className="border-green-500/30 text-green-400">
                          <CheckCircle className="w-3 h-3 mr-1" />
                          For
                        </Button>
                        <Button size="sm" variant="outline" className="border-red-500/30 text-red-400">
                          <XCircle className="w-3 h-3 mr-1" />
                          Against
                        </Button>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>

      {/* User Contributions */}
      <Card className="cyber-card">
        <div className="space-y-6">
          <h2 className="text-2xl font-semibold">Your Contributions</h2>
          
          <div className="space-y-4">
            {userContributions.map((contribution, index) => (
              <div key={index} className="flex items-center justify-between p-4 bg-muted/20 rounded-lg">
                <div className="flex items-center space-x-4">
                  <div className="p-2 bg-primary/20 rounded-lg">
                    <Vote className="w-5 h-5 text-primary" />
                  </div>
                  <div>
                    <h4 className="font-medium">{contribution.title}</h4>
                    <p className="text-sm text-muted-foreground">{contribution.dao} • {contribution.type}</p>
                  </div>
                </div>
                
                <div className="text-right">
                  <div className="flex items-center space-x-2">
                    <Badge 
                      variant={contribution.status === "Approved" ? "default" : "outline"}
                      className={contribution.status === "Approved" ? "bg-green-500/20 text-green-400" : ""}
                    >
                      {contribution.status}
                    </Badge>
                    <span className="text-primary font-medium">{contribution.reward}</span>
                  </div>
                  <p className="text-xs text-muted-foreground">{contribution.date}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>
    </div>
  );
};

export default DAOs;
