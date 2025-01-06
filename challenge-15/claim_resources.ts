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
    TokenIdentifierValue,
    AddressValue,
    StringValue,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
// const WOOD_SC = "erd1qqqqqqqqqqqqqpgqcs54gq36d6lgc4q57jpsu8veys7xe27k6dkqaswnpz";
// const WOOD_SC_TEST = "erd1qqqqqqqqqqqqqpgqrqds9gxytvqxmtu09qvjkw4r97z497de6dkqv2xnkx";
// const FOOD_SC_TEST = "erd1qqqqqqqqqqqqqpgqrad0al0gqpnqg68e9t4zfg853urg97au6dkqjz6mh3";
// const STONE_SC_TEST = "erd1qqqqqqqqqqqqqpgq23y2x76aawjjwtfdmuuqsecshvwq459m6dkqy5jyqh";
// const STONE_SC = "erd1qqqqqqqqqqqqqpgqvl3xlxz4rarxn6m95trqffkl7gwpxd7k6dkqzcpqvj";
// const FOOD_SC = "erd1qqqqqqqqqqqqqpgqqtsm6hkf89nq49z0ztys8ulr7z5gp5426dkqnaac6q";
// const GOLD_SC = "erd1qqqqqqqqqqqqqpgqggjxlqw9v9uxqn8yknm8k85ss6l5wexc6dkqjdk8r8";
// const GOLD_SC_TEST = "erd1qqqqqqqqqqqqqpgq5uujszsw0n2tvqccedjxvtl6lsau7atu6dkqhnu7jn";

const RESOURCE_WOOD_SC = "erd1qqqqqqqqqqqqqpgq38rjkuy3twesvmm39xhd95yte7n250jp6dkqzrmwma";

const FUNCTION = "claimResources";
const CHAIN_ID = "D";

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function claimResources(
    signer: UserSigner,
    resourceType: string,
  ): Promise<void> {  
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);
  
    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);

    let args = [new StringValue(resourceType)];

    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(RESOURCE_WOOD_SC),
        function: FUNCTION,
        gasLimit: BigInt(5000000),
        arguments: args,
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

async function main() {
    try {
      const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${0}_${1}.json`);
      
      const signer = await loadWallet(walletPath);
      await claimResources(signer, "WOOD");

      console.log("Resources has been claimed successfully");
    } catch (error) {
      console.error("Error during claimed resources:", error);
    }
}

main();