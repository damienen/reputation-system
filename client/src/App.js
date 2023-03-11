import { NotificationModal, SignTransactionsModals, TransactionsToastList } from "@multiversx/sdk-dapp/UI";
import { DappProvider } from "@multiversx/sdk-dapp/wrappers";
import React from "react";
import { BrowserRouter as Router } from "react-router-dom";
import "./App.css";
import Content from "./components/Content";
import Footer from "./components/Footer";
import Navbar from "./components/Navbar";

function App() {
  return (
    <Router>
      <DappProvider
        environment="devnet"
        customNetworkConfig={{
          name: "customConfig",
          apiTimeout: 6000,
          walletConnectV2ProjectId: "a5a07b89bf08093c43938c81a8f1a937",
        }}>
        <TransactionsToastList />
        <NotificationModal />
        <SignTransactionsModals className="custom-class-for-modals" />
        <div className="flex flex-col h-[100dvh] bg-slate-200">
          <Navbar />
          <Content />
          <Footer />
        </div>
      </DappProvider>
    </Router>
  );
}

export default App;
