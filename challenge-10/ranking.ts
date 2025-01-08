import axios from 'axios';
import * as fs from 'fs';

const API_BASE_URL = 'https://devnet-api.multiversx.com';
const DELAY_MS = 500;

interface TokenHolder {
    address: string;
    balance: string;
}

async function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function fetchWinterTokens(): Promise<string[]> {
    try {
        console.log('Fetching WINTER tokens...');
        let allTokens: any[] = [];
        let size = 100;
        let from = 0;
        
        while (true) {
            const url = `${API_BASE_URL}/tokens?from=${from}&size=${size}`;
            const response = await axios.get(url);
            const tokens = response.data;
            
            if (!tokens || tokens.length === 0) break;
            
            allTokens = allTokens.concat(tokens);
            if (tokens.length < size) break;
            
            from += size;
            await sleep(DELAY_MS);
        }
        
        const winterTokens = allTokens
            .filter(token => token.identifier.toUpperCase().startsWith('WINTER-'))
            .map(token => token.identifier)
            .slice(0, 10);
            
        console.log(`Found ${winterTokens.length} WINTER tokens`);
        return winterTokens;
    } catch (error) {
        console.error('Error fetching tokens:', error);
        return [];
    }
}

async function fetchTopHolders(tokenId: string): Promise<TokenHolder[]> {
    try {
        console.log(`Fetching holders for ${tokenId}...`);
        let allHolders: TokenHolder[] = [];
        let size = 100;
        let from = 0;
        
        while (true) {
            const url = `${API_BASE_URL}/tokens/${tokenId}/accounts?from=${from}&size=${size}`;
            const response = await axios.get(url);
            const holders = response.data;
            
            if (!holders || holders.length === 0) break;
            
            allHolders = allHolders.concat(holders.map((h: any) => ({
                address: h.address,
                balance: h.balance
            })));
            
            if (holders.length < size) break;
            
            from += size;
            await sleep(DELAY_MS);
        }
        
    const sortedHolders = allHolders.sort((a, b) => {
        const balanceA = BigInt(a.balance);
        const balanceB = BigInt(b.balance);
        return balanceB > balanceA ? 1 : balanceB < balanceA ? -1 : 0;
    });

    console.log(`Found ${sortedHolders.length} total holders for ${tokenId}`);
    return sortedHolders;
    } catch (error) {
        console.error(`Error fetching holders for ${tokenId}:`, error);
        return [];
    }
}

async function generateLeaderboard() {
    console.log('Starting leaderboard generation...');
    
    const tokens = await fetchWinterTokens();
    if (tokens.length === 0) {
        console.error('No WINTER tokens found!');
        return;
    }
    
    let leaderboardData: any[] = [];
    
    for (const tokenId of tokens) {
        const holders = await fetchTopHolders(tokenId);
        const top10 = holders.slice(0, 10);
        
        top10.forEach((holder, index) => {
            leaderboardData.push({
                token: tokenId,
                rank: index + 1,
                address: holder.address,
                balance: holder.balance
            });
        });
        
        console.log(`Processed ${tokenId}: Found ${holders.length} holders, added top 10 to leaderboard`);
    }
    
    const csvHeader = 'Token,Rank,Address,Balance\n';
    const csvRows = leaderboardData.map(entry => 
        `${entry.token},${entry.rank},${entry.address},${entry.balance}`
    );
    
    const csvContent = csvHeader + csvRows.join('\n');
    fs.writeFileSync('winter_leaderboard.csv', csvContent);
    console.log(`Saved leaderboard to winter_leaderboard.csv (${leaderboardData.length} entries)`);
    
    fs.writeFileSync('winter_leaderboard.json', JSON.stringify(leaderboardData, null, 2));
    console.log('Saved leaderboard to winter_leaderboard.json');
}

console.log('Script started');
generateLeaderboard()
    .then(() => console.log('Script completed successfully'))
    .catch(error => {
        console.error('Script failed:', error);
        process.exit(1);
    });