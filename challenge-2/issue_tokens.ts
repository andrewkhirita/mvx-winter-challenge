import path from 'path';
import fs from 'fs';
import { UserSigner } from '@multiversx/sdk-wallet';
import {
    Address,
    ApiNetworkProvider,
    TokenManagementTransactionsFactory,
    TransactionsFactoryConfig,
    TransactionComputer,
    Account
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";

async function issueToken(
    signer: UserSigner,
    tokenName: string,
    tokenTicker: string
): Promise<void> {
    const apiNetworkProvider = new ApiNetworkProvider(URL, { 
        clientName: "Multiversx Winter Coding" 
    });
    let config = new TransactionsFactoryConfig({ chainID: "D" });
    let tokenManagementFactory = new TokenManagementTransactionsFactory({ config: config });
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);
    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);

    const transaction = tokenManagementFactory.createTransactionForIssuingFungible({
        sender: address,
        tokenName: tokenName,
        tokenTicker: tokenTicker,
        initialSupply: BigInt(100_000_000) * BigInt(10 ** 8),
        numDecimals: BigInt(8),
        canFreeze: true,
        canWipe: true,
        canPause: true,
        canChangeOwner: true,
        canUpgrade: true,
        canAddSpecialRoles: true
    });

    const nonce = account.getNonceThenIncrement();
    transaction.nonce = BigInt(nonce.valueOf());
    const transactionComputer = new TransactionComputer();
    const serializedTransaction = transactionComputer.computeBytesForSigning(transaction);
    const signature = await signer.sign(serializedTransaction);
    transaction.signature = signature;
    const txHash = await apiNetworkProvider.sendTransaction(transaction);
    console.log("Transaction hash:", txHash);
}

async function loadWallet(walletPath: string): Promise<UserSigner> {
    const walletJson = JSON.parse(fs.readFileSync(walletPath, 'utf8'));
    return UserSigner.fromWallet(walletJson, 'password');
}

async function processWallet(shard: number, wallet: number) {
    const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${shard}_${wallet}.json`);
    const signer = await loadWallet(walletPath);
    
    for (let tokenIndex = 1; tokenIndex <= 3; tokenIndex++) {
        const tokenName = `WinterMVX${shard}${wallet}${tokenIndex}`;
        const tokenTicker = `WINTER`;
        console.log(`Processing wallet_shard${shard}_${wallet}.json with token ${tokenName}`);
        await issueToken(signer, tokenName, tokenTicker);
        await new Promise(resolve => setTimeout(resolve, 7000));
    }
}

async function main() {
    try {
        const walletPromises = [];
        for (let shard = 0; shard <= 2; shard++) {
            for (let wallet = 1; wallet <= 3; wallet++) {
                walletPromises.push(processWallet(shard, wallet));
            }
        }
        await Promise.all(walletPromises);
        console.log("All tokens have been issued successfully");
    } catch (error) {
        console.error("Error during token issuance:", error);
    }
}

main();