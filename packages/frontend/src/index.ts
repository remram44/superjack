import {Game} from 'superjack';
import {Ui} from './game';

let game: Game | undefined = undefined;
const ui: Ui = new Ui(document.getElementById('game')!);

document.getElementById('menu-single')!.addEventListener('click', () => {
  // Hide menu
  document.getElementById('menu')!.style.setProperty('display', 'none');

  // Create new game
  game = new Game();
  ui.newGame(game, game.players[0], game.players[1]);
});
