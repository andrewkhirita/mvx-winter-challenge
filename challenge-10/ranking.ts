import axios from 'axios';
import * as fs from 'fs';

interface TokenHolder {
    address: string;
    balance: string;
}

interface TokenHolders {
    [tokenIdentifier: string]: TokenHolder[];
}

async function fetchTokenIdentifiers(): Promise<string[]> {
    try {
        console.log('Fetching tokens from API...');
        let allTokens: any[] = [];
        let size = 100; 
        let from = 0;
        
        while (true) {
            const url = `https://devnet-api.multiversx.com/tokens?from=${from}&size=${size}`;
            console.log(`Fetching page from=${from}, size=${size}`);
            
            const response = await axios.get(url);
            const tokens = response.data;
            
            if (!tokens || tokens.length === 0) {
                break;
            }

            if (tokens.length > 0) {
                console.log('Sample token data:', {
                    first: tokens[0].identifier,
                    last: tokens[tokens.length - 1].identifier
                });
            }
            
            allTokens = allTokens.concat(tokens);
            if (tokens.length < size) {
                break;
            }
            
            from += size;
            await new Promise(resolve => setTimeout(resolve, 500));
        }

        console.log('Total tokens received:', allTokens.length);
        
        const winterTokens = allTokens
            .filter((token: any) => {
                const isWinter = token.identifier.toUpperCase().startsWith('WINTER-');
                if (isWinter) {
                    console.log('Found WINTER token:', token.identifier);
                }
                return isWinter;
            })
            .map((token: any) => token.identifier);
            
        console.log('Found WINTER tokens:', winterTokens);
        return winterTokens;
    } catch (error) {
        console.error('Error fetching token identifiers:', error);
        if (axios.isAxiosError(error)) {
            console.error('Response data:', error.response?.data);
            console.error('Response status:', error.response?.status);
        }
        throw error;
    }
}

async function getTokenHolders(identifier: string): Promise<TokenHolder[]> {
    try {
        console.log(`Fetching holders for ${identifier}...`);
        let allHolders: TokenHolder[] = [];
        let size = 100; // holders per page
        let from = 0;
        
        while (true) {
            const url = `https://devnet-api.multiversx.com/tokens/${identifier}/accounts?from=${from}&size=${size}`;
            console.log('Request URL:', url);
            
            const response = await axios.get(url);
            const holders = response.data;
            
            if (!holders || holders.length === 0) {
                break;
            }
            
            allHolders = allHolders.concat(holders);
            if (holders.length < size) {
                break;
            }
            
            from += size;
            await new Promise(resolve => setTimeout(resolve, 500));
        }
        
        console.log(`Received total ${allHolders.length} holders for ${identifier}`);
        return allHolders;
    } catch (error) {
        console.error(`Error fetching holders for ${identifier}:`, error);
        if (axios.isAxiosError(error)) {
            console.error('Response data:', error.response?.data);
            console.error('Response status:', error.response?.status);
        }
        return [];
    }
}

async function generateLeaderboard() {
    console.log('Starting leaderboard generation...');
    
    const tokenIdentifiers = await fetchTokenIdentifiers();
    console.log(`Found ${tokenIdentifiers.length} WINTER tokens`);
    
    if (tokenIdentifiers.length === 0) {
        console.error('No WINTER tokens found. Exiting...');
        return;
    }
    
    const allHolders: TokenHolders = {};
    
    for (const identifier of tokenIdentifiers) {
        const holders = await getTokenHolders(identifier);
        
        if (holders.length > 0) {
            allHolders[identifier] = holders
                .sort((a, b) => {
                    const balanceA = BigInt(a.balance);
                    const balanceB = BigInt(b.balance);
                    if (balanceB < balanceA) return -1;
                    if (balanceB > balanceA) return 1;
                    return 0;
                })
                .slice(0, 10); // Păstrăm doar top 10
            
            console.log(`Processed ${holders.length} holders for ${identifier}`);
        } else {
            console.log(`No holders found for ${identifier}`);
        }
        
        await new Promise(resolve => setTimeout(resolve, 1000));
    }
    
    if (Object.keys(allHolders).length === 0) {
        console.error('No data to save. Exiting...');
        return;
    }
    
    console.log('Generating CSV file...');
    let csv = 'Token Identifier,Rank,Address,Balance\n';
    
    for (const [identifier, holders] of Object.entries(allHolders)) {
        holders.forEach((holder, index) => {
            csv += `${identifier},${index + 1},${holder.address},${holder.balance}\n`;
        });
    }
    
    const filename = 'winter_tokens_leaderboard.csv';
    try {
        fs.writeFileSync(filename, csv);
        console.log(`Leaderboard has been generated: ${filename}`);
        console.log(`File saved with ${csv.split('\n').length - 1} rows`);
    } catch (error) {
        console.error('Error saving file:', error);
    }
}

console.log('Script started');
generateLeaderboard()
    .then(() => console.log('Script completed'))
    .catch(error => {
        console.error('Script failed:', error);
        process.exit(1);
    });