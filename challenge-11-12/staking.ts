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
    TokenTransfer,
    BigIntValue,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqkal99mqd6sz48hvwp4wyzad7hq28ena06dkqsrcpsj";
const FUNCTION = "stakeTokenWinter";
const CHAIN_ID = "D";

const TOKEN_ID = "WINTER-4b4989"; // Replace with your token ID
const AMOUNT = "0.000001"; // 100 tokens with 18 decimals

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
      18
    );
  
    const transaction = factory.createTransactionForExecute({
      sender: address,
      contract: Address.fromBech32(SMART_CONTRACT),
      function: FUNCTION,
      gasLimit: BigInt(500000000),
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
  
async function loadWallet(walletPath: string): Promise<UserSigner> {
    const walletJson = JSON.parse(fs.readFileSync(walletPath, 'utf8'));
    return UserSigner.fromWallet(walletJson, 'password');
}

async function main() {
    try {
      const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${0}_${1}.json`);
      
      const signer = await loadWallet(walletPath);
      await stakeTokenWinter(signer);

      console.log("Tokens have been burned successfully");
    } catch (error) {
      console.error("Error during burn token:", error);
    }
}

main();