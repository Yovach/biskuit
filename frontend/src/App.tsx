import { createSignal, type Component, Match, Switch } from 'solid-js';
import { vstack } from '../styled-system/patterns';
import { css } from '../styled-system/css';
import { LoginForm } from './components/login-form';

const App: Component = () => {
  const [jwt, setJwt] = createSignal<string | null>(null);
  console.log(jwt());
  return (
    <div class={vstack({ gap: 4, backgroundColor: "slate.950", minH: "100vh", })}>
      <Switch>
        <Match when={jwt() !== null}>

        </Match>

        <Match when={jwt() === null}>
          <span class={css({ fontWeight: "bold", fontSize: "xl", color: 'slate.300', mt: "8" })}>Login</span>
          <LoginForm onLogin={setJwt} />
        </Match>
      </Switch>
    </div>
  );
};

export default App;
