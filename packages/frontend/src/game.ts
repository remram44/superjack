import {
  Card,
  Game,
  Event,
  PlayerKnownHand,
  BasePlayer,
  Action,
  PlayGem,
  getSuitEmoji,
} from 'superjack';

const ASPECT = 1.6;

function getCardSrText(card: Card) {
  return `${card.face} of ${card.suit}`;
}

function makeCardElement(ui: Ui, card: Card): HTMLElement {
  const element = document.createElement('div');
  element.setAttribute(
    'class',
    'card card-' + card.suit + ' card-' + card.face
  );
  element.innerHTML =
    '<div class="sr-only">' +
    getCardSrText(card) +
    '</div>' +
    '<div aria-hidden="true" class="face-corner">' +
    card.face +
    '</div>' +
    '<div aria-hidden="true" class="suit-corner">' +
    getSuitEmoji(card.suit) +
    '</div>' +
    '<div aria-hidden="true" class="suit-center">' +
    getSuitEmoji(card.suit) +
    '</div>';
  element.addEventListener('click', () => ui.onCardClicked(card));
  return element;
}

function makeUnknownCardElement(): HTMLElement {
  const element = document.createElement('div');
  element.setAttribute('class', 'card card-back');
  element.innerHTML = '<img src="logo.png" alt="unrevealed card" />';
  return element;
}

interface AnimatedCard {
  card: Card | undefined;
  element: HTMLElement;
  target: [number, number];
}

interface Playing {
  game: Game;
  us: PlayerKnownHand;
  them: BasePlayer;
}

export class Ui {
  root: HTMLElement;

  // Debug elements
  rootEnemyDeck: HTMLElement;
  rootEnemyHand: HTMLElement;
  rootEnemyGems: HTMLElement;
  rootEnemyCreatures: HTMLElement;
  rootOwnCreatures: HTMLElement;
  rootOwnGems: HTMLElement;
  rootOwnHand: HTMLElement;
  rootOwnDeck: HTMLElement;

  enemyHandHeight = 10;
  ownHandHeight = 10;
  enemyGemsHeight = 10;
  enemyCreaturesHeight = 10;
  ownCreaturesHeight = 10;
  ownGemsHeight = 10;

  enemyHandTop = 10;
  enemyGemsTop = 10;
  enemyCreaturesTop = 10;
  ownCreaturesTop = 10;
  ownGemsTop = 10;
  ownHandTop = 10;

  playing: Playing | undefined;

  fontSize = 10;
  cards: Map<String, AnimatedCard> = new Map();

  constructor(root: HTMLElement) {
    this.root = root;
    this.rootEnemyDeck = document.getElementById('enemy-deck')!;
    this.rootEnemyHand = document.getElementById('enemy-hand')!;
    this.rootEnemyGems = document.getElementById('enemy-gems')!;
    this.rootEnemyCreatures = document.getElementById('enemy-creatures')!;
    this.rootOwnCreatures = document.getElementById('own-creatures')!;
    this.rootOwnGems = document.getElementById('own-gems')!;
    this.rootOwnHand = document.getElementById('own-hand')!;
    this.rootOwnDeck = document.getElementById('own-deck')!;

    this.resize = this.resize.bind(this);
    window.addEventListener('resize', this.resize);
    this.resize();
  }

  newGame(game: Game, us: PlayerKnownHand, them: BasePlayer) {
    this.playing = {game, us, them};

    // Delete previous cards
    document.querySelectorAll('.card').forEach(card => {
      card.remove();
    });
    this.cards = new Map();

    this.show();
  }

  show() {
    // Initialize UI
    this.root.style.display = '';
    this.resize();

    // Our turn?
    if (this.playing) {
      if (this.playing.game.currentPlayer === this.playing.us.id) {
        document.documentElement.classList.add('select-card');
      } else {
        document.documentElement.classList.remove('select-card');
      }
    }
  }

  resize() {
    const width = this.root.clientWidth;
    const height = this.root.clientHeight;
    this.fontSize = Math.min(width, height * ASPECT) / 80;
    this.root.style.setProperty('font-size', '' + this.fontSize + 'px');

    this.enemyHandHeight = Math.min(height * 0.2, (width / ASPECT) * 0.2);
    this.ownHandHeight = Math.min(height * 0.25, (width / ASPECT) * 0.25);
    const rest = height - this.enemyHandHeight - this.ownHandHeight;
    this.enemyGemsHeight = 0.2 * rest;
    this.enemyCreaturesHeight = 0.25 * rest;
    this.ownCreaturesHeight = 0.3 * rest;
    this.ownGemsHeight = 0.25 * rest;

    this.enemyHandTop = 0;
    this.enemyGemsTop = this.enemyHandTop + this.enemyHandHeight;
    this.enemyCreaturesTop = this.enemyGemsTop + this.enemyGemsHeight;
    this.ownCreaturesTop = this.enemyCreaturesTop + this.enemyCreaturesHeight;
    this.ownGemsTop = this.ownCreaturesTop + this.ownCreaturesHeight;
    this.ownHandTop = this.ownGemsTop + this.ownGemsHeight;

    // This is just for debugging
    let currentTop = 0;
    function position(elem: HTMLElement, height: number) {
      elem.style.setProperty('height', height + 'px');
      elem.style.setProperty('top', currentTop + 'px');
      currentTop += height;
    }
    position(this.rootEnemyHand, this.enemyHandHeight);
    position(this.rootEnemyGems, this.enemyGemsHeight);
    position(this.rootEnemyCreatures, this.enemyCreaturesHeight);
    position(this.rootOwnCreatures, this.ownCreaturesHeight);
    position(this.rootOwnGems, this.ownGemsHeight);
    position(this.rootOwnHand, this.ownHandHeight);

    // Position cards
    const cardHeight = 7 * this.fontSize;

    // Show decks
    this.rootOwnDeck.style.setProperty(
      'top',
      this.ownHandTop + (this.ownHandHeight - cardHeight) * 0.5 + 'px'
    );
    this.rootOwnDeck.style.setProperty('left', this.fontSize + 'px');
    this.rootOwnDeck.style.setProperty('height', 7 * this.fontSize + 'px');
    this.rootOwnDeck.style.setProperty('width', 5 * this.fontSize + 'px');
    this.rootEnemyDeck.style.setProperty(
      'top',
      this.enemyHandTop + (this.enemyHandHeight - cardHeight) * 0.5 + 'px'
    );
    this.rootEnemyDeck.style.setProperty('left', this.fontSize + 'px');
    this.rootEnemyDeck.style.setProperty('height', 7 * this.fontSize + 'px');
    this.rootEnemyDeck.style.setProperty('width', 5 * this.fontSize + 'px');

    if (this.playing) {
      // Our hand
      this.rootOwnDeck.innerHTML = this.playing.us.getCardsInDeck() + ' cards';
      for (let i = 0; i < this.playing.us.hand.length; ++i) {
        const card = this.playing.us.hand[i];
        const key = card.key();
        let element;
        if (this.cards.has(key)) {
          element = this.cards.get(key)!.element;
        } else {
          element = makeCardElement(this, card);
        }
        const targetX = (8 + 6 * i) * this.fontSize;
        const targetY =
          this.ownHandTop + (this.ownHandHeight - cardHeight) * 0.5;
        element.style.setProperty('left', targetX + 'px');
        element.style.setProperty('top', targetY + 'px');
        this.rootOwnHand.appendChild(element);
        this.cards.set(key, {card, element, target: [targetX, targetY]});
      }

      // Opponent's hand
      this.rootEnemyDeck.innerHTML =
        this.playing.them.getCardsInDeck() + ' cards';
      for (let i = 0; i < this.playing.them.getCardsInHand(); ++i) {
        const key = 'H' + i;
        let element;
        if (this.cards.has(key)) {
          element = this.cards.get(key)!.element;
        } else {
          element = makeUnknownCardElement();
        }
        const targetX = (8 + 6 * i) * this.fontSize;
        const targetY =
          this.enemyHandTop + (this.enemyHandHeight - cardHeight) * 0.5;
        element.style.setProperty('left', targetX + 'px');
        element.style.setProperty('top', targetY + 'px');
        this.rootEnemyHand.appendChild(element);
        this.cards.set(key, {
          card: undefined,
          element,
          target: [targetX, targetY],
        });
      }
    }
  }

  play(action: Action) {
    // TODO
  }

  onGameEvent(event: Event) {
    if (!this.playing) {
      return;
    }

    if (event.eventType === 'turn_start') {
      if (event.player === this.playing.us.id) {
        document.documentElement.classList.add('select-card');
      }
    } else if (event.eventType === 'play_gem') {
      // TODO
    } else if (event.eventType === 'draw_card') {
      // TODO
    } else {
      console.error('Unknown event type ', event.eventType);
    }
  }

  onCardClicked(card: Card) {
    if (!this.playing) {
      return;
    }
    console.log('Card clicked: ', card.toString());
    if (
      this.playing.game.currentPhase === 'play_gem' &&
      this.playing.game.currentPlayer === this.playing.us.id
    ) {
      // Play a gem
      const action: PlayGem = {
        actionType: 'play_gem',
        card: card,
      };
      this.play(action);
    }
  }
}