import { css, cx } from "../../styled-system/css";
import { VStack } from "../../styled-system/jsx";
import { vstack } from "../../styled-system/patterns";
import { inputStyles, labelStyles } from "./common/input";

type FormEvent = Event & {
  submitter: HTMLElement;
} & {
  currentTarget: HTMLFormElement;
  target: Element;
}

type Props = { onLogin: (jwt: string) => void }
export function LoginForm({ onLogin }: Props) {
  const onSubmit = async (e: FormEvent) => {
    e.preventDefault();

    const data = new FormData(e.currentTarget);

    console.log(e.currentTarget);

    const req = await fetch(import.meta.env.VITE_LOGIN_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: data.get("username"),
        password: data.get("password"),
      }),
    });
    const res = await req.json();
    if (res.data?.jwt) {
      onLogin(res.data?.jwt);
    }
  };

  return (
    <form onSubmit={onSubmit} class={vstack({
      gap: "6",
      padding: "4",
      bg: { base: "slate.100", _osDark: "slate.900" },
      borderRadius: "xl"
    })}>
      <VStack gap="0.5">
        <label class={cx(labelStyles, css({ color: "slate.200", fontWeight: "medium", alignSelf: "flex-start" }))}>Login</label>
        <input class={inputStyles} name="username" type="text" />
      </VStack>
      <VStack gap="0.5">
        <label class={cx(labelStyles, css({ color: "slate.200", fontWeight: "medium", alignSelf: "flex-start" }))}>Password</label>
        <input class={inputStyles} name="password" type="password" />
      </VStack>
      <button class={css({
        backgroundColor: "blue.500",
        alignSelf: "flex-end",
        px: "3",
        py: "1.5",
        color: "blue.100",
        borderRadius: "lg",
        fontWeight: "semibold",
      })}>
        Envoyer
      </button>
    </form>
  );
}
