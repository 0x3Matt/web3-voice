
const express = require('express');
const cors = require('cors');
const dotenv = require('dotenv');
const nearRoutes = require('./routes/near');
const ipfsRoutes = require('./routes/ipfs');
const aiRoutes = require('./routes/ai');

dotenv.config();

const app = express();
const PORT = process.env.PORT || 5000;

// Middleware
app.use(cors());
app.use(express.json());

// Routes
app.use('/near', nearRoutes);
app.use('/ipfs', ipfsRoutes);
app.use('/ai', aiRoutes);

// Test route
app.get('/', (req, res) => {
    res.send('Web3Voice Backend API is running!');
});

// Start the server
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});