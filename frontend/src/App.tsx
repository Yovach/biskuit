import { createSignal, type Component, Match, Switch } from 'solid-js';
import { css } from '../styled-system/css';
import { LoginForm } from './components/login-form';
import { CreateShortUrlForm } from './components/create-short-url-form';
import { VStack } from '../styled-system/jsx';

const App: Component = () => {
  const [jwt, setJwt] = createSignal<string | null>(null);
  return (
    <VStack
      rowGap={4}
      bg={{
        base: "slate.50",
        _osDark: "slate.950",
      }}
      fontFamily="inter"
      minH={"100vh"}
    >
      <Switch>
        <Match when={jwt() === null}>
          <span class={css({ fontWeight: "bold", fontSize: "xl", color: 'slate.300', mt: "8" })}>Login</span>
          <LoginForm onLogin={setJwt} />
        </Match>

        <Match when={jwt() !== null}>
          <CreateShortUrlForm jwt={jwt() as string} />
        </Match>
      </Switch>
    </VStack>
  );
};

export default App;
