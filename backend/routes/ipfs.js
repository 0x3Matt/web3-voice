const express = require('express');
const multer = require('multer');
const axios = require('axios');
const router = express.Router();
const dotenv = require('dotenv');
dotenv.config();

const upload = multer();

// Endpoint to upload a file to IPFS using Pinata
router.post('/upload', upload.single('file'), async (req, res) => {
    try {
        const file = req.file;
        if (!file) {
            return res.status(400).json({ success: false, error: 'No file uploaded' });
        }
        const url = 'https://api.pinata.cloud/pinning/pinFileToIPFS';
        const formData = new FormData();
        formData.append('file', file.buffer, file.originalname);

        const response = await axios.post(url, formData, {
            maxContentLength: Infinity,
            maxBodyLength: Infinity,
            headers: {
                'Content-Type': `multipart/form-data; boundary=${formData._boundary}`,
                pinata_api_key: process.env.PINATA_API_KEY,
                pinata_secret_api_key: process.env.PINATA_SECRET_API_KEY,
            },
        });

        res.json({ success: true, cid: response.data.IpfsHash });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

module.exports = router;
