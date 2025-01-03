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
    Token,
    TokenTransfer,
    U64Value,
    BigIntValue,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgq9hvgwd5dpa8md9hw73uf557z8xjuwua46dkqunflzv";

const FUNCTION_TO_CLAIM = "claimShield";
const FUNCTION_TO_MINT = "mintShield";
const FUNCTION_TO_ISSUE = "issue";

const CHAIN_ID = "D";

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

  let args = [new StringValue(tokenName),new StringValue(tokenTicker)];
  
  const transaction = factory.createTransactionForExecute({
      sender: address,
      contract: Address.fromBech32(SMART_CONTRACT),
      function: FUNCTION_TO_ISSUE,
      gasLimit: BigInt(80000000),
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

async function mintShield(
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
        function: FUNCTION_TO_MINT,
        gasLimit: BigInt(5000000),
        tokenTransfers: [
          new TokenTransfer({
              token: new Token({ identifier: "ORE-165ad2"}),
              amount: BigInt(2),
          })
      ]    });
    
    const nonce = account.getNonceThenIncrement();
    transaction.nonce = BigInt(nonce.valueOf());
  
    const transactionComputer = new TransactionComputer();
    const serializedTransaction = transactionComputer.computeBytesForSigning(transaction);
    const signature = await signer.sign(serializedTransaction);
    transaction.signature = signature;
  
    const txHash = await apiNetworkProvider.sendTransaction(transaction);
    console.log("Transaction hash:", txHash);
  }

  async function claimShield(
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
        function: FUNCTION_TO_CLAIM,
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
    //   await issueToken(signer, "TOOL", "TOOL");
    //   await mintShield(signer);
      await claimShield(signer);
      
      console.log("Proccess to mint shield was completed!");
    } catch (error) {
      console.error("Error during minting shield:", error);
    }
}

main();