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
    TokenTransfer,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
// const WOOD_SC = "erd1qqqqqqqqqqqqqpgqcs54gq36d6lgc4q57jpsu8veys7xe27k6dkqaswnpz";
// const STONE_SC = "erd1qqqqqqqqqqqqqpgqvl3xlxz4rarxn6m95trqffkl7gwpxd7k6dkqzcpqvj";
// const FOOD_SC = "erd1qqqqqqqqqqqqqpgqqtsm6hkf89nq49z0ztys8ulr7z5gp5426dkqnaac6q";
const GOLD_SC = "erd1qqqqqqqqqqqqqpgqggjxlqw9v9uxqn8yknm8k85ss6l5wexc6dkqjdk8r8";

const FUNCTION_GENERATE = "generateResources";
const FUNCTION_STAKE = "stakeWinter";
const CHAIN_ID = "D";

const TOKEN_ID = "WINTER-4b4989";
const AMOUNT = "1000";
const numDecimals = 8;

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function stakeTokenWinter(
  signer: UserSigner,
): Promise<void> {  
  const userAddress = signer.getAddress().toString();
  const address = Address.fromBech32(userAddress);

  const account = new Account(address);
  const accountOnNetwork = await apiNetworkProvider.getAccount(address);
  account.update(accountOnNetwork);

  const payment = TokenTransfer.fungibleFromAmount(
    TOKEN_ID,
    AMOUNT,
    numDecimals
  );

  const transaction = factory.createTransactionForExecute({
    sender: address,
    contract: Address.fromBech32(GOLD_SC),
    function: FUNCTION_STAKE,
    gasLimit: BigInt(5000000),
    tokenTransfers: [payment]
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

async function generateResources(
    signer: UserSigner,
  ): Promise<void> {  
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);
  
    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);
  
    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(GOLD_SC),
        function: FUNCTION_GENERATE,
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

async function main() {
    try {
      const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${0}_${1}.json`);
      
      const signer = await loadWallet(walletPath);
      // await generateResources(signer);
      await stakeTokenWinter(signer);

      console.log("Resources has been generated resources successfully");
    } catch (error) {
      console.error("Error during generated resources:", error);
    }
}

main();