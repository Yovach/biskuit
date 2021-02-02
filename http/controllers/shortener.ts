/* URL Shortener feature */
import { Router } from "https://deno.land/x/oak@v6.5.0/mod.ts";
import { IRouteModule } from "../../libs/routes.ts";
import { retrieveShortenedURL } from "./shortener/helpers.ts";

export class ShortenerRoutes implements IRouteModule {
    initRoutes(router: Router): void {

        // Route for redirect from shortcode
        router.get('/v1/url/:token', (context) => {
            const { token } = context.params;

            // if URL is malformed
            if (token === undefined) {
                context.response.status = 400;
                context.response.body = {
                    error: "INVALID_TOKEN",
                };
                return;
            }

            // if the passed token doesn't corresponds to a shortened URL
            const data = retrieveShortenedURL(token)
            if (data === undefined) {
                context.response.status = 404;
                context.response.body = {
                    error: "INVALID_OR_EXPIRED",
                };
                return;
            }

            // if the passed token corresponds to a shortened URL, redirect to destination
            context.response.redirect(data.destination);
        });

        // Route for shorten an URL
        router.put('/v1/url/', (context) => {
            console.log(context.params)
        });
    }
}