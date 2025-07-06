const express = require('express');
const axios = require('axios');
const router = express.Router();
const dotenv = require('dotenv');
dotenv.config();

// Endpoint to process audio files
router.post('/process-audio', async (req, res) => {
    try {
        const { audioUrl } = req.body;
        if (!audioUrl) {
            return res.status(400).json({ success: false, error: 'Audio URL is required' });
        }
        const response = await axios.post(
            'https://api-inference.huggingface.co/models/openai/whisper-base',
            { inputs: audioUrl },
            {
                headers: { Authorization: `Bearer ${process.env.HUGGING_FACE_API_KEY}` },
            }
        );
        const transcription = response.data.text || 'No transcription available';
        res.json({ success: true, transcription });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

module.exports = router;
