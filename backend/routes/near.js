const express = require('express');
const { connect, keyStores } = require('near-api-js');
const router = express.Router();
const dotenv = require('dotenv');
dotenv.config();

// Endpoint to mint an NFT
router.post('/mint-nft', async (req, res) => {
    try {
        const { accountId, metadata } = req.body;
        const near = await connect({
            networkId: 'testnet',
            keyStore: new keyStores.UnencryptedFileSystemKeyStore(`${process.env.HOME}/.near-credentials`),
            nodeUrl: 'https://rpc.testnet.near.org',
            walletUrl: 'https://wallet.testnet.near.org',
            helperUrl: 'https://helper.testnet.near.org',
        });
        const account = await near.account(process.env.NEAR_ACCOUNT_ID);
        const result = await account.functionCall({
            contractId: process.env.NEAR_CONTRACT_ID,
            methodName: 'nft_mint',
            args: { metadata },
            gas: '30000000000000',
            attachedDeposit: '1',
        });
        res.json({ success: true, result });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

module.exports = router;
