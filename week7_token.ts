1.	Исходный код клиентского скрипта для создания токена (⁠tests/token.ts⁠):
import * as anchor from "@coral-xyz/anchor";
import { 
  createMint, 
  getOrCreateAssociatedTokenAccount, 
  mintTo, 
  transfer 
} from "@solana/spl-token";

describe("spl_token_experiment", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  let mint: anchor.web3.PublicKey;
  let aliceTokenAccount: anchor.web3.PublicKey;
  let bobTokenAccount: anchor.web3.PublicKey;

  const recipient = anchor.web3.Keypair.generate();

  it("Создание и управление SPL-токеном", async () => {
    // 1. Создание нового минта токена (9 знаков после запятой)
    mint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9
    );
    console.log("Адрес созданного токена (Mint):", mint.toBase58());

    // 2. Создание токен-аккаунта для отправителя (Alice)
    aliceTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey
    );

    // 3. Эмиссия (минтинг) 1000 токенов на баланс Alice
    await mintTo(
      provider.connection,
      payer.payer,
      mint,
      aliceTokenAccount.address,
      payer.publicKey,
      1000 * 10 ** 9
    );
    console.log("Успешно выпущено 1000 токенов!");

    // 4. Создание токен-аккаунта для получателя (Bob)
    bobTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      recipient.publicKey
    );

    // 5. Перевод 100 токенов от Alice к Bob
    await transfer(
      provider.connection,
      payer.payer,
      aliceTokenAccount.address,
      bobTokenAccount.address,
      payer.publicKey,
      100 * 10 ** 9
    );
    console.log("Успешно переведено 100 токенов пользователю Bob!");
  });
});
