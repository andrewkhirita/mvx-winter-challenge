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
  StringValue,
  AddressValue
} from '@multiversx/sdk-core';

const URL = "https://devnet-api.multiversx.com";
const SMART_CONTRACT = "erd1qqqqqqqqqqqqqpgqw2guuvqhze7pz3kexjc84dlsq7tym3776dkq5mkptd";
const FUNCTION = "setRewardRecipient";
const CHAIN_ID = "D";

const apiNetworkProvider = new ApiNetworkProvider(URL);
const config = new TransactionsFactoryConfig({ chainID: CHAIN_ID });
const factory = new SmartContractTransactionsFactory({ config });

async function setRewardRecipient(
  signer: UserSigner,
  recipient: Address,
): Promise<void> {
  const userAddress = signer.getAddress();
  const account = new Account(userAddress);
  const accountOnNetwork = await apiNetworkProvider.getAccount(userAddress);
  account.update(accountOnNetwork);

  let args = [new AddressValue(recipient)];

  const transaction = factory.createTransactionForExecute({
    sender: userAddress,
    contract: Address.fromBech32(SMART_CONTRACT),
    function: FUNCTION,
    gasLimit: BigInt(60000000),
    arguments: args
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
    const recipient = new Address("erd1z5cpd8vapfvescq576ptw2u9vhft574ggeglq5jx29kc4uk0zhdquvnac3");
    
    await setRewardRecipient(signer, recipient);
    console.log("New recipient was set succesfully");
  } catch (error) {
    console.error("Error setting new recipient:", error);
  }
}

main();