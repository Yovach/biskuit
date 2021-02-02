import { Router } from "https://deno.land/x/oak@v6.5.0/router.ts";

export interface IRouteModule {
    
    
    initRoutes(router: Router): void;
}