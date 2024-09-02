import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlinkmartProtocol } from "../app/src/idl/blinkmart_protocol";
import { LAMPORTS_PER_SOL, sendAndConfirmTransaction } from "@solana/web3.js";

describe("blinkmart-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);
  const authority = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.BlinkmartProtocol as Program<BlinkmartProtocol>;

  function logTx(txSignature) {
    console.log(`\nTx: https://explorer.solana.com/tx/${txSignature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
  }

  function findPda(seeds: Array<Buffer | Uint8Array>,) {
    return anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
  }

  async function airdrop(addr: anchor.web3.PublicKey) {
    const airdropSignature = await connection.requestAirdrop(addr, LAMPORTS_PER_SOL * 10)
    await connection.confirmTransaction(airdropSignature);
    await sleep(2000)
    console.log(addr.toBase58(), ": ", await connection.getBalance(addr, "confirmed"));
  }

  async function execTx(tx, signers: anchor.web3.Signer[]) {
    const txHash = await provider.sendAndConfirm(tx, signers)
    logTx(txHash)
  }

  function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  const [admin,] = findPda([Buffer.from("admin")]);
  console.log("admin: ", admin.toBase58());

  const administrator = anchor.web3.Keypair.generate();
  const treasury = anchor.web3.Keypair.generate()
  const operation = anchor.web3.Keypair.generate()
  const transactionFees = 500; // 5%

  it("Initialize", async () => {
    await airdrop(administrator.publicKey)
    await airdrop(treasury.publicKey)
    await airdrop(operation.publicKey)

    const tx = await program.methods
      .initialize({
        administrator: administrator.publicKey,
        treasury: treasury.publicKey,
        operation: operation.publicKey,
        transactionFees: transactionFees
      })
      .accountsPartial({ admin: admin, payer: administrator.publicKey })
      .transaction();
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [administrator])
  });

  let productId = "e96d3d37-9850-467d-8574-7a534d7d13f7" // max length is 40\
  const productAdministrator = anchor.web3.Keypair.generate()
  const productTreasury = anchor.web3.Keypair.generate()
  const salesPrice = new anchor.BN(LAMPORTS_PER_SOL * 0.5)
  // const inventory = new anchor.BN(100)

  // productId = anchor.utils.sha256.hash(productId)
  // console.log("productId: ", productId.toString());
  // console.log("productId Buffer from: ", Buffer.from(productId));
  // console.log("productId Uint8Array from: ", Uint8Array.from(Buffer.from(productId).subarray(0, 8)));
  // console.log(productId);
  // return
  productId = productId.slice(0, 31)

  const [product,] = findPda([Buffer.from("product"), Buffer.from(productId)]);
  console.log("product: ", product.toBase58());
  console.log("productAdministrator: ", productAdministrator.publicKey.toBase58());
  console.log("productTreasury: ", productTreasury.publicKey.toBase58());

  it("Listing", async () => {
    await airdrop(productAdministrator.publicKey)
    await airdrop(productTreasury.publicKey)

    const tx = await program.methods
      .listing({
        productId: productId,
        administrator: productAdministrator.publicKey,
        treasury: productTreasury.publicKey,
        salesPrice: salesPrice,
      })
      .accountsPartial({
        authority: productAdministrator.publicKey,
        product: product
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [productAdministrator])
  });


  const customer = anchor.web3.Keypair.generate()

  let orderId = "e42faae5-622b-4d26-b3cc-5b41fec283da" // max length is 40
  orderId = orderId.slice(0, 31)
  const [order,] = findPda([Buffer.from("order"), Buffer.from(orderId)]);
  console.log("order: ", order.toBase58());
  const orderQuantity = new anchor.BN(2);


  it("Customer buy something 1...", async () => {
    await airdrop(customer.publicKey)

    const tx = await program.methods
      .placeOrder({
        orderId: orderId,
        productId: productId,
        payer: customer.publicKey,
        orderQuantity: orderQuantity
      })
      .accountsPartial({
        product: product,
        order: order,
        admin: admin,
        payer: customer.publicKey
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [customer])
  });


  let orderId2 = "82cb1edd-fb72-4de6-81ed-74cc3116ecbe".slice(0, 31) // max length is 40
  const [order2,] = findPda([Buffer.from("order"), Buffer.from(orderId2)]);
  console.log("order2: ", order2.toBase58());


  it("Customer buy something 2...", async () => {
    const tx = await program.methods
      .placeOrder({
        orderId: orderId2,
        productId: productId,
        payer: customer.publicKey,
        orderQuantity: orderQuantity
      })
      .accountsPartial({
        product: product,
        order: order2,
        admin: admin,
        payer: customer.publicKey
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [customer])
  });


  it("Customer confirm recepit", async () => {
    const tx = await program.methods
      .confirmReceipt({
        orderId: orderId,
        productId: productId,
      })
      .accountsPartial({
        payer: customer.publicKey,
        product: product,
        order: order,
        admin: admin,
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [customer])

  });


  it("Seller withdraw all", async () => {
    const tx = await program.methods
      .sellerWithdraw({
        productId: productId,
      })
      .accountsPartial({
        administrator: productAdministrator.publicKey,
        treasury: productTreasury.publicKey,
        product: product,
        admin: admin,
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [productAdministrator])

  });

  it("Customer return something...", async () => {
    const tx = await program.methods
      .cancelOrder({
        orderId: orderId2,
        productId: productId,
      })
      .accountsPartial({
        payer: customer.publicKey,
        product: product,
        order: order2,
        admin: admin,
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [customer])

  });

  it("Delisting", async () => {
    const tx = await program.methods
      .delisting({
        productId: productId,
      })
      .accountsPartial({
        administrator: productAdministrator.publicKey,
        treasury: productTreasury.publicKey,
        product: product,
        admin: admin,
      })
      .instruction()
    const txs = new anchor.web3.Transaction().add(tx);
    await execTx(txs, [productAdministrator])
  });

});
