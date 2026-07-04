// build.ts
import tailwindcss from "bun-plugin-tailwindcss";

// 1. フロントエンド (React) のビルド
await Bun.build({
  entrypoints: ["./src/main.tsx"],
  outdir: "./public/dist", // ★ public/dist に出力
  naming: "[name].[ext]",
  minify: true,
  plugins: [tailwindcss()],
});

// 2. バックエンド (Hono) のビルド
await Bun.build({
  entrypoints: ["./server/index.ts"],
  outdir: "./functions",
  naming: "[[path]].ts",
  minify: true,
});

console.log("🎉 Frontend & Backend successfully built with Bun!");
