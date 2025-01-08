import axios from 'axios';
import * as fs from 'fs';

const API_BASE_URL = 'https://devnet-api.multiversx.com';
const DELAY_MS = 500;
const MAX_TRANSACTIONS = 100;

const TARGET_ADDRESSES = [
    'erd1c2x73evg5dq3f5n4n6dpw9nu4gycdg4empjphwn33g9akqalzk3qhagq53',
    'erd1djapuvw2prv3zkfzgrc7lfp3gl9c6dr95k5lwhw2ztzffhrgzqyskc6reg',
    'erd1ktajyuj5vp3t033nukvkc60zqwkv27nnmdpp2tfym3eamvk926usjkj4yu',
    'erd1raf60v6rsg78ss5nyg5es83eyqae4d97wtwc46fxnlcfng6t2mrsxp8ham',
    'erd1wavgcxq9tfyrw49k3s3h34085mayu82wqvpd4h6akyh8559pkklsknwhwh',
    'erd1lkmanj8vfnsrsf7j5wh33wr3klmvsuwsh50l9flr53hl554t27asvywhkf',
    'erd15tfgdvmngp8dwq3tk3dw75vkseqll07u7pgyyuh0yvus9vmvu47slmqk30',
    'erd1nc72387sjaue8mhm0r6vgseedjf3vkyw499k7ucauu7u73gc7crs4l56fm',
    'erd125tnlhspx09qalen6yfvcm3n2rsy46nxlay6hxmdc4qmguxgvctsa500nq',
    'erd1ksz47exjllcwrrme7k3lhql5554wnkrx8ayl9kqggpwyqrt27xes87z8tk'
];

interface Transaction {
    txHash: string;
    sender: string;
    receiver: string;
    value: string;
    fee: string;
    gasLimit: number;
    gasUsed: number;
    status: string;
    timestamp: number;
}

function logInfo(message: string) {
    const timestamp = new Date().toISOString();
    const logMessage = `${timestamp} - INFO - ${message}`;
    console.log(logMessage);
    fs.appendFileSync('query_transactions.log', logMessage + '\n');
}

function logError(message: string, error?: any) {
    const timestamp = new Date().toISOString();
    const errorDetails = error ? `\nError details: ${JSON.stringify(error, null, 2)}` : '';
    const logMessage = `${timestamp} - ERROR - ${message}${errorDetails}`;
    console.error(logMessage);
    fs.appendFileSync('query_transactions.log', logMessage + '\n');
}

async function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function getTransactionCount(address: string): Promise<number> {
    try {
        const url = `${API_BASE_URL}/accounts/${address}/transactions/count`;
        const response = await axios.get(url);
        return parseInt(response.data.toString(), 10);
    } catch (error) {
        logError(`Error fetching transaction count for ${address}:`, error);
        return 0;
    }
}

async function getAccountTransactions(address: string): Promise<Transaction[]> {
    try {
        const expectedCount = await getTransactionCount(address);
        logInfo(`Fetching transactions for address ${address} (Expected count: ${expectedCount})...`);
        
        let allTransactions: Transaction[] = [];
        let from = 0;
        
        while (true) {
            const url = `${API_BASE_URL}/accounts/${address}/transactions?from=${from}&size=${MAX_TRANSACTIONS}`;
            const response = await axios.get(url);
            const transactions = response.data;
            
            if (!transactions || transactions.length === 0) break;
            
            const formattedTransactions = transactions.map((tx: any) => ({
                txHash: tx.txHash,
                sender: tx.sender,
                receiver: tx.receiver,
                value: tx.value,
                fee: tx.fee || '0',
                gasLimit: tx.gasLimit || 0,
                gasUsed: tx.gasUsed || 0,
                status: tx.status,
                timestamp: tx.timestamp
            }));
            
            allTransactions = allTransactions.concat(formattedTransactions);
            if (transactions.length < MAX_TRANSACTIONS) break;
            
            from += MAX_TRANSACTIONS;
            await sleep(DELAY_MS);
        }
        
        if (allTransactions.length !== expectedCount) {
            logInfo(`Warning: Got ${allTransactions.length} transactions but expected ${expectedCount} for ${address}`);
        }
        
        return allTransactions;
    } catch (error) {
        logError(`Error fetching transactions for ${address}:`, error);
        return [];
    }
}

async function generateReports() {
    logInfo('Starting report generation...');
    
    const allTransactions: any[] = [];
    const BATCH_SIZE = 5;
    
    for (let i = 0; i < TARGET_ADDRESSES.length; i += BATCH_SIZE) {
        const batch = TARGET_ADDRESSES.slice(i, i + BATCH_SIZE);
        logInfo(`Processing batch ${Math.floor(i/BATCH_SIZE) + 1}/2`);
        
        const promises = batch.map(address => getAccountTransactions(address));
        const results = await Promise.all(promises);
        
        results.forEach((transactions, index) => {
            const address = batch[index];
            transactions.forEach(tx => {
                allTransactions.push({
                    ...tx,
                    account: address
                });
            });
        });
        
        await sleep(DELAY_MS);
    }
    
    //Save in CSV
    const csvHeader = 'Account,Hash,Sender,Receiver,Value,Fee,GasLimit,GasUsed,Status,Timestamp\n';
    const csvRows = allTransactions.map(tx => 
        `${tx.account},${tx.txHash},${tx.sender},${tx.receiver},${tx.value},${tx.fee},${tx.gasLimit},${tx.gasUsed},${tx.status},${tx.timestamp}`
    );
    
    const csvContent = csvHeader + csvRows.join('\n');
    fs.writeFileSync('transactions.csv', csvContent);
    logInfo(`Saved ${allTransactions.length} transactions to transactions.csv`);
    
    //Save in JSON
    fs.writeFileSync('transactions.json', JSON.stringify(allTransactions, null, 2));
    logInfo('Saved transactions to transactions.json');
}

logInfo('Script started');
generateReports()
    .then(() => logInfo('Script completed successfully'))
    .catch(error => {
        logError('Script failed:', error);
        process.exit(1);
    });