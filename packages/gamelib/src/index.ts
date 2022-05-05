export class Game {
  num: number;

  constructor() {
    this.num = 0;
  }

  increment(by: number): void {
    this.num += by;
  }
}
