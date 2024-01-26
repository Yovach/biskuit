import type { Component } from 'solid-js';
import { vstack } from '../styled-system/patterns';
import { css } from '../styled-system/css';
import { LoginForm } from './components/login-form';

const App: Component = () => {
  return (
    <div class={vstack({ gap: 4, backgroundColor: "slate.950", minH: "100vh", })}>
      <span class={css({ fontWeight: "bold", fontSize: "xl", color: 'slate.300', mt: "8" })}>Login</span>
      <LoginForm />
    </div>
  );
};

export default App;
