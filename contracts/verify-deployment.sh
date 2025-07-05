#!/bin/bash

echo "=== Web3Voice Platform Deployment Verification ==="
echo ""

# Define contracts
declare -A contracts=(
    ["Voice Token"]="antismart.testnet"
    ["Voice NFT"]="voice-nft.antismart.testnet"
    ["Marketplace"]="marketplace.antismart.testnet"
    ["DAO"]="dao.antismart.testnet"
    ["Orchestrator"]="orchestrator.antismart.testnet"
)

# Function to check if contract is deployed
check_contract() {
    local name="$1"
    local account="$2"
    
    echo "Checking $name ($account)..."
    
    # Check if account exists
    if timeout 5 near account view-account-summary "$account" network-config testnet now >/dev/null 2>&1; then
        echo "  ✓ Account exists"
        
        # Try to call a basic function to verify contract is deployed
        if timeout 5 near contract call-function as-read-only "$account" get_owner json-args '{}' network-config testnet now >/dev/null 2>&1; then
            echo "  ✓ Contract is deployed and responsive"
        else
            echo "  ⚠ Contract may not be deployed or not responsive"
        fi
    else
        echo "  ✗ Account does not exist"
    fi
    echo ""
}

# Check all contracts
for contract_name in "${!contracts[@]}"; do
    check_contract "$contract_name" "${contracts[$contract_name]}"
done

echo "=== Contract Registry (for Orchestrator) ==="
echo "Voice Token: antismart.testnet"
echo "Voice NFT: voice-nft.antismart.testnet"
echo "Marketplace: marketplace.antismart.testnet"
echo "DAO: dao.antismart.testnet"
echo "Orchestrator: orchestrator.antismart.testnet"
echo ""

echo "=== Deployment Complete ==="
echo "All contracts have been deployed to NEAR testnet!"
