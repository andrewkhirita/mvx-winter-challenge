import path from 'path';
import fs from 'fs';
import { UserSigner } from '@multiversx/sdk-wallet';
import {
    Address,
    ApiNetworkProvider,
    TransactionsFactoryConfig,
    TransactionComputer,
    Account,
    SmartContractTransactionsFactory,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqlaa66qc2uapx5ef79a4csqu2cgqpr0ty6dkqpl73p8";
const FUNCTION = "claimTokens";
const CHAIN_ID = "D";

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function claimTokens(
    signer: UserSigner,
  ): Promise<void> {  
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);
  
    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);
  
    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(SMART_CONTRACT),
        function: FUNCTION,
        gasLimit: BigInt(5000000),
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

async function claimTokensWallets(shardId: number, walletIndex: number): Promise<void> {
    try {
        const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${shardId}_${walletIndex}.json`);
        
        const signer = await loadWallet(walletPath);
        await claimTokens(signer);
        
        console.log(`Tokens was succesfully claimed!`);
    } catch (error) {
        console.error(`Error in claiming tokens!`, error);
        throw error; 
    }
}

async function main() {
    const SHARD_COUNT = 3;
    const WALLETS_PER_SHARD = 3;
    
    try {
        const claimTokensPromises: Promise<void>[] = [];
        for (let shardId = 0; shardId < SHARD_COUNT; shardId++) {
            for (let walletIndex = 1; walletIndex <= WALLETS_PER_SHARD; walletIndex++) {
                claimTokensPromises.push(claimTokensWallets(shardId, walletIndex));
            }
        }
        await Promise.all(claimTokensPromises);        
        console.log("All tokens have been claimed successfully!");
    } catch (error) {
        console.error("Error during parallel token claims:", error);
    }
}

main();