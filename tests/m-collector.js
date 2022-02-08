const assert = require("assert");
const anchor = require('@project-serum/anchor');
// Need the system program, will talk about this soon.
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.MemeCollector;

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Meme Count', account.totalMemes.toString())

  // Call add_meme!
  await program.rpc.addMeme("https://media.giphy.com/media/rWiEbamfqOHrq/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Meme Count', account.totalMemes.toString())

  // Access gif_list on the account!
  console.log('👀 Meme List', account.memeList)
  console.log('👀 Trying to upvote meme', account.memeList)

  await program.rpc.upvoteMeme('0', {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Access gif_list on the account!
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Meme List', account.memeList)
  console.log('👀 Trying to upvote again', account.memeList)

  await program.rpc.upvoteMeme('0', {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Access gif_list on the account!
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Meme List', account.memeList)
  console.log('👀 Trying to upvote nonexisting meme, should print error and continue')

  try {
    const tx = await program.rpc.upvoteMeme('1000', {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    // assert.ok(false);
  } catch (err) {
    const errMsg = "Meme not found";
    assert.equal(err.toString(), errMsg);
  }
  

  // Access gif_list on the account!
  console.log('👀 Meme List', account.memeList)
}

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