import { Game } from 'superjack';
import * as React from 'react';

interface GameComponent {
  increment(num: number): Promise<void>;
}

interface GameState {
  num: number;
  value: string;
}

class LocalGame extends React.PureComponent<{}, GameState> implements GameComponent {
  game: Game;

  constructor(props: {}) {
    super(props);
    this.game = new Game();
    this.state = {num: this.game.num, value: ''};
  }

  async increment(num: number) {
    this.game.increment(num);
    this.setState({num: this.game.num});
  }

  render() {
    return (
      <div className="game game-local">
        <p>Num: {this.state.num}</p>
        <p>
          <input value={this.state.value} onChange={(evt) => this.setState({value: evt.target.value})} />
          <input type="button" onClick={() => { this.increment(parseInt(this.state.value, 10)); this.setState({value: ''}); }} value="Increment" />
        </p>
      </div>
    );
  }
}

class NetworkGame extends React.PureComponent<{}, GameState> implements GameComponent {
  constructor(props: {}) {
    super(props);
    this.state = {num: 0, value: ''};

    (async () => {
      const response = await fetch('/api');
      if(response.status === 200) {
        const obj = await response.json();
        this.setState({num: obj.num});
      }
    })();
  }

  async increment(num: number) {
    const response = await fetch(
      '/api',
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json; charset=utf-8',
        },
        body: JSON.stringify({type: 'increment', num: num}),
      },
    );
    if(response.status !== 200) {
      throw new Error('Status ' + response.status);
    }
    const obj = await response.json();
    this.setState({num: obj.num});
  }

  render() {
    return (
      <div className="game game-network">
        <p>Num: {this.state.num}</p>
        <p>
          <input value={this.state.value} onChange={(evt) => this.setState({value: evt.target.value})} />
          <input type="button" onClick={() => { this.increment(parseInt(this.state.value, 10)); this.setState({value: ''}); }} value="Increment" />
        </p>
      </div>
    );
  }
}

export function App(): React.ReactElement {
  const [count, setCount] = React.useState(0);

  return (
    <div>
      <h1>Local game</h1>
      <LocalGame />
      <h1>Network game</h1>
      <NetworkGame />
    </div>
  );
}
