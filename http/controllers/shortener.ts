/* URL Shortener feature */
import { Router, Status } from "https://deno.land/x/oak@v6.5.0/mod.ts";
import { IRouteModule } from "../../libs/routes.ts";
import { retrieveShortenedURL } from "./shortener/helpers.ts";

export class ShortenerRoutes implements IRouteModule {
    initRoutes(router: Router): void {

        // Route for redirect from shortcode
        router.get('/v1/url/:token', (context) => {
            const { token } = context.params;

            // if URL is malformed
            if (token === undefined) {
                context.throw(Status.BadRequest)
                return;
            }


            // if the passed token doesn't corresponds to a shortened URL
            const data = retrieveShortenedURL(token)
            if (data === undefined) {
                context.throw(Status.NotFound)
                return;
            }

            // if the passed token corresponds to a shortened URL, redirect to destination
            context.response.redirect(data.destination);
        });

        // Route for shorten an URL
        router.put('/v1/url/', (context) => {

            if (!context.request.hasBody) {
                context.throw(Status.BadRequest)
            }

            const body = context.request.body();

            console.log(body)

            // console.log(context.request.body().value.url)
        });
    }
}