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
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqjrdlyg4spc6jmyxe2aa56a7w5uyqd0wg6dkq3gf9mx";

const FUNCTION = "mintCitizen";
const CHAIN_ID = "D";

const EGLD_FEE = 50000000000000000;
const TOKEN_NAME = "CITIZEN";
const TICKER = "CTZ";

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

async function mintCitizen(
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

async function main() {
    try {
      const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${0}_${1}.json`);
      
      const signer = await loadWallet(walletPath);
      await mintCitizen(signer);

      const tokenName = `${TOKEN_NAME}`;
      const tokenTicker = `${TICKER}`;
      
      // await issueToken(signer, tokenName, tokenTicker);

      console.log("Resources has been generated resources successfully");
    } catch (error) {
      console.error("Error during generated resources:", error);
    }
}

main();