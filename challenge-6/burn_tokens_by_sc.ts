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
    U8Value,
    StringValue,
    BigUIntValue,
    BooleanValue,
    TokenIdentifierValue,
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqlaa66qc2uapx5ef79a4csqu2cgqpr0ty6dkqpl73p8";
const FUNCTION = "burnTokens";

const TICKER = "SNOW-44d7a4";
const NONCE = 0; //standard for all esdts
const CHAIN_ID = "D";

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function burnTokens(
    signer: UserSigner,
    tokenIdentifier: TokenIdentifierValue,
    amount: BigUIntValue,
  ): Promise<void> {
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);
  
    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);
  
    let args = [tokenIdentifier, new U8Value(NONCE), amount];
    
    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(SMART_CONTRACT),
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
      const tokenTicker = `${TICKER}`;
      
      const signer = await loadWallet(walletPath);
      await burnTokens(signer, new TokenIdentifierValue(tokenTicker), new BigUIntValue(500000000000000000));

      console.log("Tokens have been burned successfully");
    } catch (error) {
      console.error("Error during burn token:", error);
    }
}

main();