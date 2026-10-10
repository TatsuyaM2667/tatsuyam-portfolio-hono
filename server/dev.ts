// server/dev.ts
// Local Bun server: serves the built WASM bundle from ./dist and mounts the API.
// Usage: bun run server/dev.ts   (or `bun run dev:server`)
import { serveStatic } from "hono/bun";
import { app } from "./index";

const root = "./dist";
const port = Number(process.env.PORT ?? 8787);

app.use("*", serveStatic({ root }));

app.get("*", async (c) => {
  const { pathname } = new URL(c.req.url);
  if (pathname.includes(".")) return c.text("Not Found", 404);
  const file = Bun.file(`${root}/index.html`);
  if (!(await file.exists())) return c.text("Run `bun run build:fe` first.", 503);
  return c.html(await file.text());
});

Bun.serve({ port, fetch: app.fetch });
console.log(`▲ portfolio → http://localhost:${port}`);
