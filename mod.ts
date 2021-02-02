import { Application, isHttpError, Router } from "https://deno.land/x/oak@v6.5.0/mod.ts";
import { ShortenerRoutes } from "./http/controllers/shortener.ts";

const router = new Router();
const app = new Application();

app.use(async (context, next) => {
  try {
    await next();
  } catch (err) {
    if (isHttpError(err)) {
      context.response.status = err.status;
    }
  }
})


app.use(router.routes());
app.use(router.allowedMethods());

const activeModules = [ShortenerRoutes].map((routeModule) => new routeModule());

activeModules.forEach((routeModule) => {
  routeModule.initRoutes(router);
})

await app.listen({ port: 8000 });