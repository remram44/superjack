import {Card, Game, PlayerKnownHand, BasePlayer, getSuitEmoji} from 'superjack';

const ASPECT = 1.6;

function getCardSrText(card: Card) {
  return `${card.face} of ${card.suit}`;
}

function makeCardElement(card: Card): HTMLElement {
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
  return element;
}

function makeUnknownCardElement(): HTMLElement {
  const element = document.createElement('div');
  element.setAttribute('class', 'card card-back');
  element.innerHTML = '<img src="logo.png" alt="unrevealed card" />';
  return element;
}

export class Ui {
  root: HTMLElement;
  rootEnemyHand: HTMLElement;
  rootEnemyGems: HTMLElement;
  rootEnemyCreatures: HTMLElement;
  rootOwnCreatures: HTMLElement;
  rootOwnGems: HTMLElement;
  rootOwnHand: HTMLElement;

  enemyHandHeight: number;
  ownHandHeight: number;
  enemyGemsHeight: number;
  enemyCreaturesHeight: number;
  ownCreaturesHeight: number;
  ownGemsHeight: number;

  enemyHandTop: number;
  enemyGemsTop: number;
  enemyCreaturesTop: number;
  ownCreaturesTop: number;
  ownGemsTop: number;
  ownHandTop: number;

  game: Game | undefined;
  us: PlayerKnownHand | undefined;
  them: BasePlayer | undefined;

  fontSize = 10;
  cards: Map<String, {card: Card; element: HTMLElement}> = new Map();

  constructor(root: HTMLElement) {
    this.root = root;
    this.rootEnemyHand = document.getElementById('enemy-hand')!;
    this.rootEnemyGems = document.getElementById('enemy-gems')!;
    this.rootEnemyCreatures = document.getElementById('enemy-creatures')!;
    this.rootOwnCreatures = document.getElementById('own-creatures')!;
    this.rootOwnGems = document.getElementById('own-gems')!;
    this.rootOwnHand = document.getElementById('own-hand')!;

    this.resize = this.resize.bind(this);
    window.addEventListener('resize', this.resize);
    this.resize();
  }

  newGame(game: Game, us: PlayerKnownHand, them: BasePlayer) {
    this.game = game;
    this.us = us;
    this.them = them;
    this.show();

    // Delete previous cards
    document.querySelectorAll('.card').forEach((card: HTMLElement) => {
      card.remove();
    });

    // Create cards
    this.cards = new Map();
    const cardHeight = 7 * this.fontSize;

    // Our hand
    if (this.us) {
      for (let i = 0; i < this.us.hand.length; ++i) {
        const card = this.us.hand[i];
        const key = card.key();
        const element = makeCardElement(card, this.us.id);
        element.style.setProperty('left', '' + (2 + 6 * i) + 'em');
        element.style.setProperty(
          'top',
          '' +
            (this.ownHandTop + (this.ownHandHeight - cardHeight) * 0.5) +
            'px'
        );
        this.rootOwnHand.appendChild(element);
        this.cards.set(key, {card, element});
      }
    }
  }

  show() {
    // Initialize UI
    this.root.style.display = '';
    this.resize();
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
      elem.style['height'] = '' + height + 'px';
      elem.style['top'] = '' + currentTop + 'px';
      currentTop += height;
    }
    position(this.rootEnemyHand, this.enemyHandHeight);
    position(this.rootEnemyGems, this.enemyGemsHeight);
    position(this.rootEnemyCreatures, this.enemyCreaturesHeight);
    position(this.rootOwnCreatures, this.ownCreaturesHeight);
    position(this.rootOwnGems, this.ownGemsHeight);
    position(this.rootOwnHand, this.ownHandHeight);
  }
}
