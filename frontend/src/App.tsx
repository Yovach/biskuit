import type { Component } from 'solid-js';
import { vstack } from '../styled-system/patterns';

const App: Component = () => {
  return (
    <div class={vstack({ gap: "4" })}>
      <span>Create your URL</span>
      <form>
      </form>
    </div>
  );
};

export default App;
