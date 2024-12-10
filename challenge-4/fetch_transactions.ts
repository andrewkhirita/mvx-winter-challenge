import { ApiNetworkProvider } from '@multiversx/sdk-network-providers';
import { Address } from '@multiversx/sdk-core';
import * as fs from 'fs';  
import * as fsPromises from 'fs/promises'; 
import path from 'path';

const URL = "https://devnet-api.multiversx.com";
const provider = new ApiNetworkProvider(URL, { 
    clientName: "Multiversx Winter Coding" 
});
const outputDir = "./transactions";

async function fetchTransactions(addressStr: string | Address | Uint8Array) {
    const address = new Address(addressStr);
    let transactions = [];
    let index = 0;
    const limit = 100;

    while (true) {
        const url = `accounts/${address.bech32()}/transactions?from=${index}&size=${limit}`;
        const result = await provider.doGetGeneric(url);
        
        if (!result || result.length === 0) break;
        
        transactions.push(...result);
        console.log(`Fetched ${transactions.length} transactions`);
        
        if (result.length < limit) break;
        index += limit;
    }

    return transactions;
}

async function saveToFile(transactions: string | any[], shard: number, wallet: number) {
    const filename = `${outputDir}/transactions_shard${shard}_${wallet}.json`;
    await fsPromises.writeFile(filename, JSON.stringify(transactions), 'utf-8');
    console.log(`Saved ${transactions.length} transactions to ${filename}`);
}

async function getWalletAddress(shard: number, wallet: number) {
    const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${shard}_${wallet}.json`);
    const walletData = JSON.parse(await fsPromises.readFile(walletPath, 'utf-8'));  // async
    return walletData.bech32;
}

const setupOutputDirectory = (dirPath: string) => {
    if (!fs.existsSync(dirPath)) {
        fs.mkdirSync(dirPath);
    }
};

async function main() {
    for (let shard = 0; shard <= 2; shard++) {
        for (let wallet = 1; wallet <= 3; wallet++) {
            try {
                const address = await getWalletAddress(shard, wallet);
                console.log(`Processing wallet ${shard}_${wallet}: ${address}`);
                const transactions = await fetchTransactions(address);
                setupOutputDirectory('./transactions');
                await saveToFile(transactions, shard, wallet);
            } catch (error) {
                console.error(`Error processing wallet ${shard}_${wallet}:`, error);
            }
        }
    }
}

main();