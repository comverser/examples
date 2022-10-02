import React, { useEffect, useState } from "react";
import { shortenAddress, useEthers, useLookupAddress } from "@usedapp/core";
import styles from "../styles";

const WalletButton = () => {
  const [accountAddress, setAccountAddress] = useState("");

  const { ens } = useLookupAddress();
  const { account, activateBrowserWallet, deactivate } = useEthers();

  useEffect(() => {
    if (ens) {
      setAccountAddress(ens);
      console.debug("useEffect: ens", ens);
    } else if (account) {
      setAccountAddress(shortenAddress(account));
      console.debug("useEffect: No ens");
    } else {
      setAccountAddress("");
      console.debug("useEffect: No account");
    }
  }, [account, ens, setAccountAddress]);

  return (
    <button
      onClick={() => {
        console.debug("account", account);
        if (!account) {
          activateBrowserWallet();
        } else {
          deactivate();
        }
      }}
      className={styles.walletButton}
    >
      {accountAddress || "Connect Wallet"}
    </button>
  );
};

export default WalletButton;
