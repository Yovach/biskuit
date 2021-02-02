import { Application, Router } from "https://deno.land/x/oak@v6.5.0/mod.ts";
import { ShortenerRoutes } from "./http/controllers/shortener.ts";

const router = new Router();
const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

const activeModules = [ShortenerRoutes].map((routeModule) => new routeModule());

activeModules.forEach((routeModule) => {
  routeModule.initRoutes(router);
  console.log(routeModule.toString())
})

await app.listen({ port: 8000 });