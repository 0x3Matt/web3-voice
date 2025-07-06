import axios from 'axios';

// TODO: Replace with your deployed backend URL
const BASE_URL = 'https://web3voice-backend.onrender.com';

export const mintNFT = async (accountId, metadata) => {
    try {
        const response = await axios.post(`${BASE_URL}/near/mint-nft`, { accountId, metadata });
        return response.data;
    } catch (error) {
        console.error('Error minting NFT:', error);
        throw error;
    }
};

export const uploadFileToIPFS = async (file) => {
    try {
        const formData = new FormData();
        formData.append('file', file);
        const response = await axios.post(`${BASE_URL}/ipfs/upload`, formData, {
            headers: { 'Content-Type': 'multipart/form-data' },
        });
        return response.data;
    } catch (error) {
        console.error('Error uploading file to IPFS:', error);
        throw error;
    }
};

export const processAudio = async (audioUrl) => {
    try {
        const response = await axios.post(`${BASE_URL}/ai/process-audio`, { audioUrl });
        return response.data;
    } catch (error) {
        console.error('Error processing audio:', error);
        throw error;
    }
};
