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
    StringValue,
    BigUIntValue,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgq2tpr8nlmvzym0c55xmwvs3r9ktzmvc7v6dkq0cvzpx";
const FUNCTION = "issueTokenSnow";
const TOKEN_NAME = "SnowXwAs";
const TICKER = "SNOW";
const CHAIN_ID = "D";

const TOTAL_SUPPLY = 15000000000000000000;
const EGLD_FEE = 50000000000000000;

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function issueToken(
    signer: UserSigner,
    tokenName: string,
    tokenTicker: string,
): Promise<void> {
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);

    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);

    let args = [new StringValue(tokenName),new StringValue(tokenTicker), new BigUIntValue(TOTAL_SUPPLY)];
    
    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(SMART_CONTRACT),
        function: FUNCTION,
        gasLimit: BigInt(100000000),
        arguments: args,
        nativeTransferAmount: BigInt(EGLD_FEE)
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

async function issueTokenForWallet(shardId: number, walletIndex: number): Promise<void> {
    try {
        const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${shardId}_${walletIndex}.json`);
        const tokenName = `${TOKEN_NAME}${shardId}${walletIndex}`;
        const tokenTicker = `${TICKER}`;
        
        const signer = await loadWallet(walletPath);
        await issueToken(signer, tokenName, tokenTicker);
        
        console.log(`Token issued successfully for Shard ${shardId}, Wallet ${walletIndex}`);
    } catch (error) {
        console.error(`Error issuing token for Shard ${shardId}, Wallet ${walletIndex}:`, error);
        throw error; 
    }
}

async function main() {
    const SHARD_COUNT = 3;
    const WALLETS_PER_SHARD = 3;
    
    try {
        const issuancePromises: Promise<void>[] = [];
        for (let shardId = 0; shardId <= SHARD_COUNT; shardId++) {
            for (let walletIndex = 1; walletIndex <= WALLETS_PER_SHARD; walletIndex++) {
                issuancePromises.push(issueTokenForWallet(shardId, walletIndex));
            }
        }
        await Promise.all(issuancePromises);        
        console.log("All tokens have been issued successfully!");
    } catch (error) {
        console.error("Error during parallel token issuance:", error);
    }
}

main();