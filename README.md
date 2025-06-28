# Web3Voice - Decentralized Voice Content Platform

A decentralized ecosystem where human voices are tokenized, validated, and rewarded, creating a transparent and trustworthy "truth layer" for the internet.

## 🎙️ Vision

To build a decentralized ecosystem where human voices are tokenized, validated, and rewarded, creating a transparent and trustworthy "truth layer" for the internet through:

- **Voice Tokenization**: Transform authentic voice content into valuable NFTs
- **AI-Powered Insights**: Extract meaningful insights from voice data using advanced NLP
- **Decentralized Storage**: Permanent, verifiable storage on IPFS/Filecoin
- **Community Governance**: DAO-driven platform governance and rewards
- **NEAR Protocol Integration**: Secure, scalable blockchain infrastructure

## 🏗️ System Architecture

The platform consists of modular components that work together:

1. **Data Engine**: AI/NLP microservice for transcription and insight extraction
2. **Storage Layer**: Decentralized storage on IPFS/Filecoin with Arweave backup
3. **Blockchain Layer**: NEAR Protocol smart contracts for tokenization and governance
4. **Backend/API**: Unified API layer connecting frontend to decentralized backend
5. **Frontend**: React-based web application (this repository)

### Data Flow
```
X Space Audio → Data Engine → AI Analysis → IPFS Storage → NEAR Contracts → API → Frontend
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+ and npm installed - [install with nvm](https://github.com/nvm-sh/nvm#installing-and-updating)
- Git
- NEAR CLI for blockchain interaction

### Installation

```sh
# Clone the repository
git clone https://github.com/0x3Matt/web3-voice.git

# Navigate to the project directory
cd web3-voice

# Install dependencies
npm install

# Start the development server
npm run dev
```

The application will be available at `http://localhost:8080`

### NEAR Environment Setup

```sh
# Install NEAR CLI
npm install -g near-cli

# Login to NEAR testnet
near login

# Set environment variables (see .env.example)
cp .env.example .env
```

## 🛠️ Tech Stack

### Frontend (This Repository)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **UI Components**: shadcn/ui
- **State Management**: React Query
- **Routing**: React Router
- **Charts**: Recharts
- **Blockchain**: near-api-js for NEAR integration

### Backend Stack (Other Repositories)
- **Data Engine**: Python 3.9+ with FastAPI, OpenAI Whisper, Hugging Face Transformers
- **API Layer**: Node.js with Express/NestJS, REST & GraphQL endpoints
- **Blockchain**: Rust smart contracts on NEAR Protocol
- **Storage**: IPFS/Filecoin via Textile/Estuary API
- **Database**: Graph Protocol indexer + traditional DB for caching

## 📁 Project Structure

```
web3-voice/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── ui/             # shadcn/ui components  
│   │   ├── AppSidebar.tsx  # Navigation sidebar
│   │   └── DashboardLayout.tsx
│   ├── pages/              # Page components
│   │   ├── Dashboard.tsx   # Main dashboard with insights
│   │   ├── VoiceVault.tsx  # Voice asset management
│   │   ├── MintStudio.tsx  # NFT minting interface
│   │   ├── Marketplace.tsx # Voice NFT marketplace
│   │   ├── Analytics.tsx   # Performance analytics
│   │   ├── DAOs.tsx        # DAO governance
│   │   └── Settings.tsx    # User settings & wallet
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utility functions
│   └── styles/             # CSS and styling
├── public/                 # Static assets
└── docs/                   # Documentation
```

## 🎨 Features

### 📊 Dashboard
- **Insight Search**: Query AI-extracted insights by topic and sentiment
- **Thread Creation**: Tools for creating voice content threads
- **Metrics Overview**: Earnings, NFT count, listener analytics
- **VOICE Token Balance**: Real-time token balance and reward history

### 🎙️ Voice Vault
- **Voice Asset Management**: Upload, record, and organize voice files
- **AI Protection Status**: Track content protection and watermarking
- **Category Management**: Draft, Minted, Archived, DAO Submissions
- **Waveform Visualizations**: Interactive audio previews

### 🪙 Mint Studio
- **NFT Creation**: Transform voice content into blockchain assets
- **Metadata Management**: Title, tags, descriptions, sentiment analysis
- **Access Control**: Public, token-gated, or DAO-only access
- **Multi-Chain Support**: NEAR Protocol with expansion planned
- **Cost Estimation**: Real-time minting cost calculations

### 🛒 Marketplace
- **Voice NFT Discovery**: Browse and search authentic voice content
- **AI-Verified Provenance**: Blockchain-verified authenticity
- **Advanced Filtering**: By sentiment, topic, creator, price range
- **VOICE Token Payments**: Native token integration

### 📈 Analytics
- **Performance Metrics**: Detailed analytics on voice content performance
- **Geographic Distribution**: Global listener engagement maps
- **Earnings Tracking**: Revenue from NFTs, royalties, and rewards
- **AI Insights**: Content performance predictions and recommendations

### 🫂 DAOs & Governance
- **Community Governance**: Participate in platform decision-making
- **Proposal System**: Create and vote on platform improvements
- **Reward Distribution**: Transparent, community-driven rewards
- **Space Hosting**: Onboard and manage voice content spaces

## 🔧 Development

### Available Scripts

```sh
npm run dev          # Start development server
npm run build        # Build for production
npm run build:dev    # Build for development
npm run lint         # Run ESLint
npm run preview      # Preview production build
```

### Core Principles

- **Modular & API-First**: Independent components with well-defined APIs
- **Decentralization First**: Prioritize decentralized solutions
- **Community & Value**: Built to serve and empower the community
- **Security by Design**: Security best practices at every layer

### Data Structures

The platform uses a standardized **Contribution Object**:

```json
{
  "id": "uuid",
  "type": "thread | space",
  "contributor": "account_id.near",
  "metadata": {
    "topic": "string",
    "speaker": "handle", 
    "timestamp": "iso8601",
    "sentiment": "positive | neutral | negative",
    "insight": "string",
    "source_url": "string"
  },
  "storage": {
    "ipfs_cid": "string",
    "audio_cid": "string"
  },
  "likes": "int",
  "verified_influencers": "int"
}
```

## 🎨 Design System

- **Theme**: Dark mode with cyberpunk aesthetic
- **Primary Color**: Blood Moon Red (#FF0033 to #E6193C gradient)
- **Secondary**: Deep black (#0A0A0A) with ember orange highlights
- **Typography**: Modern, clean fonts with responsive sizing
- **Components**: Glassmorphism effects, glow animations, waveform visualizations

## 🔒 Security

- **Smart Contract Audits**: All contracts audited by Velnexor
- **Rate Limiting**: Robust error handling on all APIs
- **GDPR Compliance**: Privacy-first data handling
- **Decentralized Storage**: Immutable, verifiable content storage

## 🚀 Deployment

- **Primary**: Fleek or Akash for decentralized hosting
- **Fallback**: AWS for high availability
- **Monitoring**: Prometheus and Grafana for system health
- **CI/CD**: GitHub Actions for automated testing and deployment

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Follow the development guidelines in `dev-guide.md`
4. Run tests and linting (`npm run lint`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Setup

1. Clone the Web3Voice GitHub organization repository
2. Run `npm install` for dependencies
3. Set up NEAR testnet environment using `near-cli`
4. Review environment variable setup in `.env.example`
5. Run test suite to verify configuration

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/0x3Matt/web3-voice)
- [Developer Guide](./dev-guide.md)
- [Voice Features Specification](./voice-features.md)
- [NEAR Protocol](https://near.org/)
- [Web3Voice Organization](https://github.com/Web3Voice)

---

Built with ❤️ for the decentralized future of voice content and human truth.
