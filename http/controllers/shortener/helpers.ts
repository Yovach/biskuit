import { randomToken, URL_CHARACTERS } from "../../../libs/helpers.ts";

export interface ShortenedURL {
    token: string;
    destination: string;
    expireAt?: number;
}

const shortenedURLs: ShortenedURL[] = [];

/**
 * Shorten an URL
 * @param destination - URL destination
 * @param expireAt - Expiration of temporary link
 * @returns an object that contains all data about shortened URL
 */
export function shortenURL(destination: string, expireAt?: number | string): ShortenedURL {
    let data = shortenedURLs.find((item) => item.destination === destination);
    if (!data) {
        
        if (expireAt !== undefined) {
            if (typeof expireAt === "string") {
                expireAt = parseInt(expireAt, 10)
            }
            if (expireAt > Date.now()) {
                expireAt = undefined;
            }
        }

        data = {
            destination,
            expireAt,
            token: randomToken(URL_CHARACTERS, 8),
        }
        shortenedURLs.push(data);
    }
    return data;
}

/**
 * Retrieve a destination URL from a shortcode
 * @param shortcode - Shortcode that corresponds to a destination
 * @returns an object that contains all data about shortened URL if valid or "undefined"
 */
export function retrieveURL(shortcode: string): ShortenedURL | undefined {
    const data = shortenedURLs.find((item) => {
        if (item.token !== shortcode) {
            return false;
        }
        // if there is a expiration
        if (item.expireAt !== undefined) {
            return item.expireAt > Date.now()
        }
        return true;
    });
    return data;
}
