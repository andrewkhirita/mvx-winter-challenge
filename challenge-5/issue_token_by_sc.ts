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
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgquuqzmlvqz7qdgnnfl5qwt50ncxw08y70896qephqm0";
const FUNCTION = "issueTokenSnow";
const TICKER = "SNOW";
const CHAIN_ID = "D";

const TOTAL_SUPPLY = 50000000000000000000;
const EGLD_FEE = 50000000000000000;

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function issueToken(
    signer: UserSigner,
    tokenName: string,
    tokenTicker: string
): Promise<void> {
    const userAddress = signer.getAddress().toString();
    const address = Address.fromBech32(userAddress);

    const account = new Account(address);
    const accountOnNetwork = await apiNetworkProvider.getAccount(address);
    account.update(accountOnNetwork);

    let args = [new StringValue(tokenName), new StringValue(tokenTicker), new U8Value(8), new BigUIntValue(TOTAL_SUPPLY), 
        new BooleanValue(true), new BooleanValue(false), new BooleanValue(false), new BooleanValue(true), new BooleanValue(true), new BooleanValue(true)];
    
    const transaction = factory.createTransactionForExecute({
        sender: address,
        contract: Address.fromBech32(SMART_CONTRACT),
        function: FUNCTION,
        gasLimit: BigInt(60000000),
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

async function main() {
    try {
      // Using generated wallets from Shard 0 as an example to issue tokens
      const walletPath = path.join(__dirname, `../challenge-1/wallets/wallet_shard${2}_${1}.json`);
      const tokenName = `${TICKER}${2}${1}`;
      const tokenTicker = `${TICKER}`;
      
      const signer = await loadWallet(walletPath);
      await issueToken(signer, tokenName, tokenTicker);

      console.log("All tokens have been issued successfully");
    } catch (error) {
      console.error("Error during token issuance:", error);
    }
}

main();