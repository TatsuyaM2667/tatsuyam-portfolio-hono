// build.ts
export {};

// Bundles the Hono API into a Cloudflare Pages Functions entry.
// The frontend is a Rust/Leptos WASM app built separately by Trunk (`build:fe`).
await Bun.build({
  entrypoints: ["./server/index.ts"],
  outdir: "./functions",
  naming: "[[path]].ts",
  minify: true,
});

console.log("Server Bundled to functions/[[path]].ts");
