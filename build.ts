// build.ts

// 1. フロントエンド (React) のビルド
await Bun.build({
  entrypoints: ["./src/main.tsx"],
  outdir: "./public/dist",
  naming: "[name].[ext]",
  minify: true,
  // プラグインは不要になったので削除
});

// 2. バックエンド (Hono) のビルド
await Bun.build({
  entrypoints: ["./server/index.ts"],
  outdir: "./functions",
  naming: "[[path]].ts",
  minify: true,
});

console.log("🎉 JS & Backend successfully built with Bun!");
