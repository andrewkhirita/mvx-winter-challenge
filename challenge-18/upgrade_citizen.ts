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
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqxyap2nc27auqg2mwe0lmhug2hysmwtka6dkqyurgdd";

const FUNCTION_TO_UPGRADE = "upgradeCitizen";

const CHAIN_ID = "D";

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID});
const factory = new SmartContractTransactionsFactory({config:config});

async function upgradeCitizen(
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
      function: FUNCTION_TO_UPGRADE,
      gasLimit: BigInt(5000000),
      tokenTransfers: [
        new TokenTransfer({
          token: new Token({identifier: "CITIZEN-253783", nonce: BigInt(17)}), amount: BigInt(1),
      }),
        new TokenTransfer({
            token: new Token({ identifier: "GOLD-ec9de5"}),
            amount: BigInt(5),
        }),
        new TokenTransfer({
            token: new Token({ identifier: "ORE-165ad2"}),
            amount: BigInt(5),
        }),
    ]    
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
      await upgradeCitizen(signer);
      
      console.log("Citizen was upgraded!");
    } catch (error) {
      console.error("Error during upgrade citizen:", error);
    }
}

main();