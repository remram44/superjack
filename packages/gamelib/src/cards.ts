export type Suit = 'Spades' | 'Clubs' | 'Hearts' | 'Diamonds';
export const ALL_SUITS: Suit[] = ['Spades', 'Clubs', 'Hearts', 'Diamonds'];

export type Face = '2' | '3' | '4' | '5' | '6' | '7' | 'J' | 'Q' | 'K' | 'A';
export const ALL_FACES: Face[] = [
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  'J',
  'Q',
  'K',
  'A',
];

export type Color = 'Red' | 'Black';

export function getSuitColor(suit: Suit): Color {
  if (suit === 'Spades' || suit === 'Clubs') {
    return 'Black';
  } else {
    return 'Red';
  }
}

export function getSuitEmoji(suit: Suit) {
  if (suit === 'Spades') {
    return '\u2660';
  } else if (suit === 'Clubs') {
    return '\u2663';
  } else if (suit === 'Hearts') {
    return '\u2665';
  } else if (suit === 'Diamonds') {
    return '\u2666';
  } else {
    return '?';
  }
}

export class Card {
  owner: 0 | 1;
  suit: Suit;
  face: Face;

  constructor(owner: 0 | 1, suit: Suit, face: Face) {
    this.owner = owner;
    this.suit = suit;
    this.face = face;
  }

  toString() {
    return `${this.suit} ${this.face}`;
  }

  key() {
    return `${this.suit}-${this.face}-${this.owner}`;
  }
}

function shuffleArray(array: Array<unknown>) {
  for (let i = array.length - 1; i > 0; --i) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

export function randomDeck(owner: 0 | 1) {
  const deck = [];
  for (const suit of ALL_SUITS) {
    for (const face of ALL_FACES) {
      deck.push(new Card(owner, suit, face));
    }
  }
  shuffleArray(deck);
  return deck;
}
