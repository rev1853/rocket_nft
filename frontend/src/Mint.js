import './index.css';
import { ConnectWallet } from './components/ConnectWallet';
import { useConnectedWallet, useInstallChromeExtension, useLCDClient, useWallet } from '@terra-money/wallet-provider';
import { useCallback, useEffect, useMemo } from 'react';
import { MnemonicKey, MsgSend, Wallet } from '@terra-money/terra.js';

function Mint() {

    const lcdClient = useLCDClient()
    const wallet = useWallet();
    const connectedWallet = useConnectedWallet()


    const main = async () => {

        const sendMsg = new MsgSend(connectedWallet.terraAddress, 'terra1l7rxaj3nneqvnrgfxhxyqkx644k069gtw6m3jx', { uluna: 1000000 });
        const tx = await lcdClient.tx.create(
            [{
                address: connectedWallet.terraAddress,
            }],
            {
                msgs: [sendMsg]
            }
        )
        console.log(tx);
        const signResult = await connectedWallet.sign(tx)
        const broadcastResult = await lcdClient.tx.broadcast(tx);
        console.log(broadcastResult);
    }

    useEffect(() => {
        console.log(wallet.status);
        if (wallet.status != 'WALLET_CONNECTED') return;
        main();
    }, [wallet])
    return (
        <ConnectWallet />
    );
}

export default Mint;