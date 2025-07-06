# Web3Voice Backend API

This is the backend service for the Web3Voice platform. It provides RESTful endpoints for:
- Minting NFTs on NEAR Protocol
- Uploading files to IPFS via Pinata
- Processing/transcribing audio files using Hugging Face AI

## 📁 Backend Structure

```
backend/
├── index.js           # Main entry point, sets up Express and routes
├── routes/
│   ├── near.js        # NEAR Protocol endpoints (NFT minting)
│   ├── ipfs.js        # Pinata endpoints (file upload)
│   └── ai.js          # AI endpoints (audio transcription)
├── package.json       # Backend dependencies and scripts
├── .env.example       # Example environment variables
```

## 🚦 API Endpoints

### 1. NEAR Protocol
- **POST /near/mint-nft**
  - Mint a new NFT on NEAR testnet
  - **Body:**
    ```json
    {
      "accountId": "your_account_id.testnet",
      "metadata": {
        "title": "My First NFT",
        "description": "This is a test NFT",
        "media": "https://example.com/image.png"
      }
    }
    ```
  - **Response:** `{ success: true, result }` or error

### 2. Pinata IPFS Storage
- **POST /ipfs/upload**
  - Upload a file to IPFS via Pinata
  - **Body:** `form-data` with a `file` field
  - **Response:** `{ success: true, cid }` or error

### 3. AI Audio Processing
- **POST /ai/process-audio**
  - Transcribe an audio file using Hugging Face Whisper
  - **Body:**
    ```json
    {
      "audioUrl": "https://example.com/audio.mp3"
    }
    ```
  - **Response:** `{ success: true, transcription }` or error

## 🛠️ Environment Variables

Copy `.env.example` to `.env` and fill in your credentials:

```
NEAR_ACCOUNT_ID=your_account_id.testnet
NEAR_CONTRACT_ID=your_contract_id.testnet
PINATA_API_KEY=your-pinata-api-key
PINATA_SECRET_API_KEY=your-pinata-secret-api-key
HUGGING_FACE_API_KEY=your_huggingface_api_key
```

## 🏃‍♂️ Running Locally

```sh
cd backend
npm install
npm run dev
```

The backend will run on `http://localhost:5000` by default.

## 📝 Code Documentation

- All route logic is modularized in the `routes/` folder.
- Each route file is documented with comments explaining its purpose and usage.
- Error handling is implemented for all endpoints.

## 🚀 Deployment

- Deployable to Render, Vercel, or any Node.js-compatible cloud platform.
- Set environment variables in your deployment dashboard for security.

---

For more details, see the main [README.md](../README.md).
