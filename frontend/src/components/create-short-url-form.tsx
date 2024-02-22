import { createSignal } from "solid-js";
import { css } from "../../styled-system/css";
import { VStack } from "../../styled-system/jsx";
import { vstack } from "../../styled-system/patterns";

type FormEvent = Event & {
  submitter: HTMLElement;
} & {
  currentTarget: HTMLFormElement;
  target: Element;
}

type Props = { jwt: string }
export function CreateShortUrlForm({ jwt }: Props) {

  const [error, setError] = createSignal<string | null>(null);
  const [shortUrl, setShortUrl] = createSignal<string | null>(null);
  const [longUrl, setLongUrl] = createSignal<string | null>(null);

  const onSubmit = async (e: FormEvent) => {
    e.preventDefault();

    const data = new FormData(e.currentTarget);

    const req = await fetch(import.meta.env.VITE_CREATE_SHORT_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        url: data.get('url'),
        jwt: jwt,
      }),
    });
    const res = await req.json();
    if (res.error) {
      setError(res.error);
    } else {
      setLongUrl(res.data.url);
      setShortUrl("http://localhost:3000/s/" + res.data.token);
    }
  };

  return (
    <form onSubmit={onSubmit} class={vstack({ gap: "6", padding: "4", backgroundColor: "slate.900", borderRadius: "8" })}>
      <VStack gap="2">
        <label class={css({ color: "slate.200", fontWeight: "medium", alignSelf: "flex-start" })}>URL à shortener</label>
        <input name="url" type="url" />
      </VStack>
      <button class={css({ backgroundColor: "purple.500", px: "4", py: "1.5", color: "purple.100", borderRadius: "lg", fontWeight: "semibold" })}>
        Envoyer
      </button>
      {
        error() ? (
          <span class={css({ color: "red.500" })}>{error()}</span>
        ) : shortUrl() && (
          <p class={css({ color: "slate.200" })}>
            Voici ton URL raccourcie : <a class={css({ color: "blue.400" })} href={shortUrl()}>{longUrl()}</a>
          </p>
        )
      }
    </form>
  );
}
