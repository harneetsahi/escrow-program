import { before, describe, test, it } from "node:test";
import assert from "node:assert";
import { connect, Connection, ErrorWithTransaction } from "solana-kite";
import { type KeyPairSigner, type Address } from "@solana/kit";
import {
  createTestOffer,
  getRandomBigInt,
  ONE_SOL,
} from "./escrow.test-helpers";

const ACCOUNT_IN_USE_ERROR =
  "11111111111111111111111111111111.Allocate: account already in use";

describe("Escrow", () => {
  let connection: Connection;
  let user: KeyPairSigner;
  let alice: KeyPairSigner;
  let bob: KeyPairSigner;
  let tokenMintA: Address;
  let tokenMintB: Address;
  let aliceTokenAccountA: Address;
  let bobTokenAccountA: Address;
  let aliceTokenAccountB: Address;

  const tokenDecimals = 9;

  // to convert between major and minor units
  const TOKEN = 10n ** BigInt(tokenDecimals);

  const aliceInitialTokenAAmount = 10n * TOKEN;

  const bobInitialTokenAAmount = 1n;

  const bobInitialTokenBAmount = 1n * TOKEN;

  const tokenAOfferedAmount = 1n * TOKEN;
  const tokenBWantedAmount = 1n * TOKEN;

  before(async () => {
    connection = await connect();

    // 'user' will be the account we use to create the token mints
    [user, alice, bob] = await connection.createWallets(3, {
      airdropAmount: ONE_SOL,
    });

    tokenMintA = await connection.createTokenMint({
      mintAuthority: user,
      decimals: tokenDecimals,
      name: "Token A",
      symbol: "TOKEN_A",
      uri: "https://example.com/token-a",
      additionalMetadata: {
        keyOne: "valueOne",
        keyTwo: "valueTwo",
      },
    });

    tokenMintB = await connection.createTokenMint({
      mintAuthority: user,
      decimals: tokenDecimals,
      name: "Token B",
      symbol: "TOKEN_B",
      uri: "https://example.com/token-b",
      additionalMetadata: {
        keyOne: "valueOne",
        keyTwo: "valueTwo",
      },
    });

    // Mint tokens to alice and bob
    await connection.mintTokens(
      tokenMintA,
      user,
      aliceInitialTokenAAmount,
      alice.address
    );
    await connection.mintTokens(
      tokenMintA,
      user,
      bobInitialTokenAAmount,
      bob.address
    );
    await connection.mintTokens(
      tokenMintB,
      user,
      bobInitialTokenBAmount,
      bob.address
    );

    // Get the token accounts
    aliceTokenAccountA = await connection.getTokenAccountAddress(
      alice.address,
      tokenMintA,
      true
    );
    bobTokenAccountA = await connection.getTokenAccountAddress(
      bob.address,
      tokenMintA,
      true
    );
    aliceTokenAccountB = await connection.getTokenAccountAddress(
      alice.address,
      tokenMintB,
      true
    );
  });

  describe("makeOffer", () => {
    test("successfully creates an offer with valid inputs", async () => {
      const { offer, vault } = await createTestOffer({
        connection,
        maker: alice,
        tokenMintA,
        tokenMintB,
        makerTokenAccountA: aliceTokenAccountA,
        tokenAOfferedAmount,
        tokenBWantedAmount,
      });

      // verify the offer was created successfully by checking the vault balance
      const vaultBalanceResponse = await connection.getTokenAccountBalance({
        tokenAccount: vault,
        mint: tokenMintA,
        useTokenExtensions: true,
      });
      assert.equal(
        vaultBalanceResponse.amount,
        tokenAOfferedAmount,
        "Vault balance should match offered amount"
      );
    });
  });
});
