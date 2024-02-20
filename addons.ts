await Deno.openKv("https://api.deno.com/databases/bd5c2931-3c6b-494c-8097-4962a6066c70/connect");

Deno.env.set("DENO_KV_ACCESS_TOKEN", "123456789ö12");
const prefs = {
  username: "ada",
  theme: "dark",
  language: "en-US",
};

const result = await kv.set(["preferences", "ada"], prefs);