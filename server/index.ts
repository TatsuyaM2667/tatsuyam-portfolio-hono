// server/index.ts
// Runtime-agnostic Hono app shared by three entry points:
//   - Cloudflare Workers + static assets (default export)
//   - Cloudflare Pages Functions (onRequest)
//   - Bun (server/dev.ts)
import { Hono } from "hono";
import { handle } from "hono/cloudflare-pages";

type Bindings = {
  ASSETS?: { fetch(request: Request): Promise<Response> };
};

export const app = new Hono<{ Bindings: Bindings }>();

app.get("/api/status", (c) =>
  c.json({ status: "operational", engine: "Bun + Hono", runtime: "wasm/leptos" }),
);

// Optional Spotify "now playing" stub. Returning `item: null` makes the
// frontend hide the widget gracefully instead of erroring.
app.get("/api/now-playing", (c) => c.json({ item: null }));

// Cloudflare Pages Functions entry.
export const onRequest = handle(app);

// Cloudflare Workers entry (Workers + static assets).
export default {
  async fetch(request: Request, env: Bindings): Promise<Response> {
    const { pathname } = new URL(request.url);
    if (pathname.startsWith("/api/")) return app.fetch(request, env);
    return env.ASSETS
      ? env.ASSETS.fetch(request)
      : new Response("Not Found", { status: 404 });
  },
};
