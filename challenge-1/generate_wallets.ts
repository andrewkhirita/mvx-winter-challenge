import { Mnemonic, UserWallet, UserSigner } from "@multiversx/sdk-wallet";
import { Address, AddressComputer } from "@multiversx/sdk-core";
import * as fs from 'fs';

interface WalletData {
    mnemonic: Mnemonic;
    secretKey: any;
    address: string;
    shard: number;
}

const addressComputer = new AddressComputer();

const generateWallet = () => {
    const mnemonic = Mnemonic.generate();
    const addressIndex = 0;
    const secretKey = mnemonic.deriveKey(addressIndex);
    const userSigner = new UserSigner(secretKey);
    const userAddress = userSigner.getAddress().toString();
    const address = Address.fromBech32(userAddress);
    const shard = addressComputer.getShardOfAddress(address);
    return {
        mnemonic,
        secretKey,
        address: userAddress,
        shard
    };
};

const saveWalletToFile = (walletData: WalletData, filename: string, password: string) => {
    const userWallet = UserWallet.fromSecretKey({ 
        secretKey: walletData.secretKey, 
        password 
    });
    const jsonFileContent = userWallet.toJSON();
    fs.writeFileSync(filename, JSON.stringify(jsonFileContent, null, 2));
};

const generateWalletsForShard = async (
    shard: number, 
    count: number,
    outputDir: string,
    password: string
) => {
    let walletsGenerated = 0;
    while (walletsGenerated < count) {
        const walletData = generateWallet();
        if (walletData.shard === shard) {
            const filename = `${outputDir}/wallet_shard${shard}_${walletsGenerated + 1}.json`;
            saveWalletToFile(walletData, filename, password);
            walletsGenerated++;
        }
    }
};

const setupOutputDirectory = (dirPath: string) => {
    if (!fs.existsSync(dirPath)) {
        fs.mkdirSync(dirPath);
    }
};

async function main() {
    try {
        const CONFIG = {
            password: "password",
            walletsPerShard: 3,
            outputDir: './wallets',
            totalShards: 3
        };
        setupOutputDirectory(CONFIG.outputDir);
        for (let shard = 0; shard < CONFIG.totalShards; shard++) {
            await generateWalletsForShard(
                shard,
                CONFIG.walletsPerShard,
                CONFIG.outputDir,
                CONFIG.password
            );
        }
        console.log('Successfully generated all wallets!')
    } catch (error) {
        console.error('Error generating wallets:', error);
    }
}
main();