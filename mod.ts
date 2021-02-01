import { Application, Router } from "https://deno.land/x/oak@v6.5.0/mod.ts";

const router = new Router();
const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());
await app.listen({ port: 8000 });