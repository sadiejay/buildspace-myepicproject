const anchor = require('@project-serum/anchor');

const main = async() => {
  console.log("🚀 Starting test...")

  // tells anchor to automatically compile our code and deploy locally
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Myepicproject;
  const tx = await program.rpc.startStuffOff();

  console.log("📝 Your transaction signature", tx);
}

// main func will run async
const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();