import { randomToken, URL_CHARACTERS } from "../../../libs/helpers.ts";

export interface ShortenedURL {
    token: string;
    destination: string;
    expireAt?: Date;
}

const shortenedURLs: ShortenedURL[] = [];

export function createShortenedURL(destination: string, expireAt?: Date): ShortenedURL {
    let data = shortenedURLs.find((item) => item.destination === destination);
    if (!data) {
        data = {
            destination,
            expireAt,
            token: randomToken(URL_CHARACTERS, 8),
        }
        shortenedURLs.push(data);
    }
    return data;
}

export function retrieveShortenedURL(token: string): ShortenedURL | undefined {
    const data = shortenedURLs.find((item) => {
        if (item.token !== token) {
            return false;
        }
        // if there is a expiration
        if (item.expireAt !== undefined) {
            return item.expireAt.getTime() > Date.now()
        }
        return true;
    });
    return data;
}
