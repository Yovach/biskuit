/* URL Shortener feature */
import { Router, RouterContext, Status } from "https://deno.land/x/oak@v6.5.0/mod.ts";
import { IRouteModule } from "../../libs/routes.ts";
import { shortenURL, retrieveURL } from "./shortener/helpers.ts";

interface ShortData {
    expireAt: string;
    destination: string;
}

export class ShortenerRoutes implements IRouteModule {
    initRoutes(router: Router): void {

        // Route for redirect from shortcode
        router.get('/v1/url/:token', (context: RouterContext) => {
            const { token } = context.params;

            // if URL is malformed
            context.assert(typeof token !== "undefined", Status.BadRequest);
            
            const data = retrieveURL(token!)

            // if the passed token doesn't corresponds to a shortened URL
            context.assert(typeof data !== "undefined", Status.NotFound);
        
            // if the passed token corresponds to a shortened URL, redirect to destination
            context.response.redirect(data!.destination);
        });

        // Route for shorten an URL
        router.put('/v1/url/', async (context: RouterContext) => {

            if (!context.request.hasBody) {
                context.throw(Status.BadRequest)
            }


            const body = context.request.body();
            let shortDetails: Partial<ShortData> | undefined;
            if (body.type === "json") {
                shortDetails = await body.value;
            } else if (body.type === "form") {
                shortDetails = {};
                for (const [key, value] of await body.value) {
                    shortDetails[key as keyof ShortData] = value;
                }
            } else if (body.type === "form-data") {
                const formData = await body.value.read();
                shortDetails = formData.fields;
            }
            context.assert(shortDetails && typeof shortDetails.destination === "string", Status.BadRequest);

            context.response.status = Status.OK;
            context.response.body = shortenURL(shortDetails.destination!, shortDetails.expireAt);
        });
    }
}