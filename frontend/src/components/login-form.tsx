import { css } from "../../styled-system/css";
import { VStack } from "../../styled-system/jsx";
import { vstack } from "../../styled-system/patterns";

type FormEvent = Event & {
  submitter: HTMLElement;
} & {
  currentTarget: HTMLFormElement;
  target: Element;
}

export function LoginForm() {
  const onSubmit = async (e: FormEvent) => {
    e.preventDefault();

    const data = new FormData(e.currentTarget);

    console.log(e.currentTarget);

    const req = await fetch("http://192.168.1.76:3000/login/", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: data.get("username"),
        password: data.get("password"),
      }),

      // DO NOT THIS
      mode: "no-cors",
    });
    const res = await req.json();
    console.log(req, res);
  };


  return (
    <form onSubmit={onSubmit} class={vstack({ gap: "6", padding: "4", backgroundColor: "slate.900", borderRadius: "8" })}>
      <VStack gap="2">
        <label class={css({ color: "slate.200", fontWeight: "medium", alignSelf: "flex-start" })}>Login</label>
        <input name="username" type="text" />
      </VStack>
      <VStack gap="2">
        <label class={css({ color: "slate.200", fontWeight: "medium", alignSelf: "flex-start" })}>Password</label>
        <input name="password" type="password" />
      </VStack>
      <button class={css({ backgroundColor: "purple.500", px: "4", py: "1.5", color: "purple.100", borderRadius: "lg", fontWeight: "semibold" })}>
        Envoyer
      </button>
    </form>
  );
}
