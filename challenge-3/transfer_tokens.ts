import path from 'path';
import fs from 'fs';
import { UserSigner } from '@multiversx/sdk-wallet';
import {
    Address,
    ApiNetworkProvider,
    TransactionsFactoryConfig,
    TransactionComputer,
    TransferTransactionsFactory,
    TokenTransfer,
    Token,
    Account
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const CHAIND_ID = "D";
const TICKER = "WINTER";

const apiNetworkProvider = new ApiNetworkProvider(URL, { 
    clientName: "Multiversx Winter Coding" 
});

async function fetchAccounts(totalAccounts: number): Promise<Address[]> {
    const allAddresses: Address[] = [];
    const size = 200;
    let from = 0;

    while (allAddresses.length < totalAccounts) {
        const response = await fetch(
            `${URL}/accounts?from=${from}&size=${size}&isSmartContract=false`
        );
        const accounts = await response.json();
        if (accounts.length === 0) break;
        const addresses = accounts.map((acc: { address: string }) => Address.fromBech32(acc.address));
        allAddresses.push(...addresses);
        from += size;

        await new Promise(resolve => setTimeout(resolve, 200));
    }
    return allAddresses.slice(0, totalAccounts);
}

async function getTokenIdentifier(walletAddress: Address): Promise<string> {
    const tokens = await apiNetworkProvider.getFungibleTokensOfAccount(walletAddress);
    const myToken = tokens.find(token => token.identifier.startsWith(`${TICKER}-`));
    if (!myToken) {
        throw new Error(`No ${TICKER} token found for wallet ${walletAddress.toString()}`);
    }
    return myToken.identifier;
}

async function processAllWallets(walletPaths: string[], receiverAddresses: Address[]) {
    const batchSize = 100; 
    const batchDelay = 20000;

    const promises = walletPaths.map(async (walletPath) => {
        console.log(`Starting transactions for wallet: ${walletPath}`);
        for (let i = 0; i < receiverAddresses.length; i += batchSize) {
            const batch = receiverAddresses.slice(i, i + batchSize);
            console.log(`Processing batch ${i / batchSize + 1} for wallet: ${walletPath}`);
            await processWallet(walletPath, batch);

            if (i + batchSize < receiverAddresses.length) {
                await new Promise(resolve => setTimeout(resolve, batchDelay));
            }
        }
        console.log(`Finished transactions for wallet: ${walletPath}`);
    });

    await Promise.all(promises);
    console.log('All wallets finished processing');
}

async function processWallet(walletPath: string, receiverAddresses: Address[]) {
    const signer = await UserSigner.fromWallet(
        JSON.parse(fs.readFileSync(walletPath, 'utf8')), 
        'password'
    );
    const apiNetworkProvider = new ApiNetworkProvider(URL);
    const senderAddress = Address.fromBech32(signer.getAddress().toString());
    const tokenIdentifier = await getTokenIdentifier(senderAddress);
    const networkAccount = await apiNetworkProvider.getAccount(senderAddress);
    const account = new Account(senderAddress);
    account.update({
        nonce: networkAccount.nonce,
        balance: networkAccount.balance
    });

    const factory = new TransferTransactionsFactory({ 
        config: new TransactionsFactoryConfig({ chainID: CHAIND_ID }) 
    });

    const transactions = await Promise.all(receiverAddresses.map(async (receiverAddress) => {
        const transaction = factory.createTransactionForESDTTokenTransfer({
            sender: senderAddress,
            receiver: receiverAddress,
            tokenTransfers: [
                new TokenTransfer({
                    token: new Token({ identifier: tokenIdentifier }),
                    amount: BigInt(10_000_00000000),
                })
            ]
        });
        const nonce = account.getNonceThenIncrement();
        transaction.nonce = BigInt(nonce.valueOf());
        
        const serializedTransaction = new TransactionComputer()
            .computeBytesForSigning(transaction);
        const signatureBuffer = await signer.sign(serializedTransaction);
        transaction.signature = new Uint8Array(signatureBuffer);
        
        return transaction;
    }));

    const txHashes = await apiNetworkProvider.sendTransactions(transactions);
    return txHashes.length;
}

async function main() {
    try {
        const receiverAddresses = await fetchAccounts(1000);
        const walletPaths: string[] = [];

        for (let shard = 0; shard <= 2; shard++) {
            for (let wallet = 1; wallet <= 3; wallet++) {
                const walletPath = path.join(
                    __dirname, 
                    `../challenge-1/wallets/wallet_shard${shard}_${wallet}.json`
                );
                walletPaths.push(walletPath);
            }
        }
        console.log(`Processing wallets: ${walletPaths.length} wallets`);
        await processAllWallets(walletPaths, receiverAddresses);
    } catch (error) {
        console.error('Error:', error);
    }
}

main();
