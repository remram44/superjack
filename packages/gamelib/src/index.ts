import {Card, randomDeck} from './cards';
export {Card, getSuitEmoji} from './cards';

export interface BasePlayer {
  id: 0 | 1;
  getCardsInDeck(): number;
  getCardsInHand(): number;
  discard: Card[];
}

export interface PlayerKnownHand extends BasePlayer {
  hand: Card[];
}

// The local player in a multiplayer game
//
// Knows their hand but not their deck.
export class LocalPlayer implements BasePlayer, PlayerKnownHand {
  id: 0 | 1;
  cardsInDeck: number;
  hand: Card[];
  discard: Card[];

  getCardsInDeck(): number {
    return this.cardsInDeck;
  }

  getCardsInHand(): number {
    return this.hand.length;
  }

  constructor(id: 0 | 1, cardsInDeck: number, hand: Card[], discard: Card[]) {
    this.id = id;
    this.cardsInDeck = cardsInDeck;
    this.hand = hand;
    this.discard = discard;
  }
}

// The other player in a multiplayer game
//
// We don't know their hand.
export class RemotePlayer implements BasePlayer {
  id: 0 | 1;
  cardsInDeck: number;
  cardsInHand: number;
  discard: Card[];

  getCardsInDeck(): number {
    return this.cardsInDeck;
  }

  getCardsInHand(): number {
    return this.cardsInHand;
  }

  constructor(
    id: 0 | 1,
    cardsInDeck: number,
    cardsInHand: number,
    discard: Card[]
  ) {
    this.id = id;
    this.cardsInDeck = cardsInDeck;
    this.cardsInHand = cardsInHand;
    this.discard = discard;
  }
}

// A player in a local game
//
// Everything is known.
export class Player implements BasePlayer, PlayerKnownHand {
  id: 0 | 1;
  deck: Card[];
  hand: Card[];
  discard: Card[] = [];

  getCardsInHand(): number {
    return this.hand.length;
  }

  getCardsInDeck(): number {
    return this.deck.length;
  }

  constructor(id: 0 | 1, deck: Card[], hand: Card[]) {
    this.id = id;
    this.deck = deck;
    this.hand = hand;
  }
}

export interface BaseGame {
  players: [BasePlayer, BasePlayer];
  currentPlayer: 0 | 1;
}

export class Game implements BaseGame {
  players: [Player, Player];
  currentPlayer: 0 | 1;

  constructor() {
    const players = [];
    for (const i of [0, 1] as (0 | 1)[]) {
      // Get a random deck
      const deck = randomDeck(i);

      // Draw 7 cards
      const hand = [];
      for (let j = 0; j < 7; ++j) {
        hand.push(deck.pop()!);
      }

      // Create player
      players.push(new Player(i, deck, hand));
    }
    this.players = players as [Player, Player];

    this.currentPlayer = 0;
  }
}
