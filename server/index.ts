// server/index.ts
import { Hono } from "hono";
import { serveStatic } from "hono/cloudflare-pages";

const app = new Hono();

// 1. API エンドポイント (ここは共通)
app.get("/api/status", (c) => {
  return c.json({ status: "operational", engine: "Bun + Hono" });
});

// 2. Cloudflare Pages Functions 用のエクスポート
// (hono/cloudflare-pages の handle はビルド時用なので、型安全にエクスポートする形にします)
import { handle } from "hono/cloudflare-pages";
export const onRequest = handle(app);

// 3. ★ローカル開発（Bun で直接動かす時）のための静的ファイル配信設定
// これにより Wrangler なしで Hono が public/ 内の index.html や dist を配れます
if (
  typeof process !== "undefined" &&
  process.versions &&
  process.versions.bun
) {
  const { serveStatic: bunServeStatic } = await import("hono/bun");

  // 静的ファイルを配信
  app.use("*", bunServeStatic({ root: "./public" }));

  // SPA用のフォールバック (ルート以外のリクエストも index.html に流す)
  app.get("*", async (c) => {
    const file = Bun.file("./public/index.html");
    return c.html(await file.text());
  });
}

export default app;
