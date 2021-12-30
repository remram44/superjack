use rand::{Rng, thread_rng};

#[derive(Debug)]
enum Error {
    Exit,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Error::Exit => write!(f, "User requested exit"),
            &Error::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            &Error::Exit => None,
            &Error::Io(ref e) => Some(e),
        }
    }
}

fn main() {
    match Game::play() {
        Ok(()) => {}
        Err(Error::Exit) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn read_yes_no(prompt: &str, default: Option<bool>) -> Result<bool, Error> {
    let options = match default {
        None => "y/n",
        Some(true) => "Y/n",
        Some(false) => "y/N",
    };
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    loop {
        print!("{} [{}] ", prompt, options);
        match stdin.read_line(&mut buffer) {
            Ok(0) => return Err(Error::Exit),
            Err(e) => return Err(Error::Io(e)),
            Ok(_) => {
                let response = buffer.trim().to_lowercase();
                if response == "yes" || response == "y" {
                    return Ok(true);
                } else if response == "no" || response == "n" {
                    return Ok(false);
                } else if response == "" && default.is_some() {
                    return Ok(default.unwrap());
                } else {
                    println!("I didn't understand {:?}", buffer.trim());
                }
            }
        }
        buffer.clear();
    }
}

fn read_number(prompt: &str, max: i32, cancellable: bool) -> Result<i32, Error> {
    let options = match cancellable {
        true => format!("1-{} or 0", max),
        false => format!("1-{}", max),
    };
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    loop {
        print!("{} [{}] ", prompt, options);
        match stdin.read_line(&mut buffer) {
            Ok(0) => return Err(Error::Exit),
            Err(e) => return Err(Error::Io(e)),
            Ok(_) => {
                let response = buffer.trim();
                if response == "" && cancellable {
                    return Ok(0);
                }
                match response.parse::<i32>() {
                    Ok(0) if cancellable => return Ok(0),
                    Ok(i) if 1 <= i && i <= max => {
                        return Ok(i);
                    }
                    _ => {
                        println!("Invalid value");
                    }
                }
            }
        }
        buffer.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Face {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Jack,
    Queen,
    King,
    Ace,
}

impl Face {
    fn all() -> &'static [Face] {
        &[
            Face::Two,
            Face::Three,
            Face::Four,
            Face::Five,
            Face::Six,
            Face::Seven,
            Face::Jack,
            Face::Queen,
            Face::King,
            Face::Ace,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn all() -> &'static [Suit] {
        &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Color::Red => write!(f, "red"),
            Color::Black => write!(f, "black"),
        }
    }
}

impl Suit {
    fn color(&self) -> Color {
        match *self {
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::Hearts | Suit::Diamonds => Color::Red,
        }
    }
}

#[derive(Debug)]
struct Card {
    player: u32,
    suit: Suit,
    face: Face,
}

impl Card {
    fn color(&self) -> Color {
        self.suit.color()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let face = match self.face {
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Jack => "jack",
            Face::Queen => "queen",
            Face::King => "king",
            Face::Ace => "ace",
        };
        let suit = match self.suit {
            Suit::Spades => "spades",
            Suit::Hearts => "hearts",
            Suit::Diamonds => "diamonds",
            Suit::Clubs => "clubs",
        };
        write!(f, "{} of {}", face, suit)
    }
}

#[derive(Debug, Clone, Copy)]
enum CreatureStatus {
    Ready,
    Tapped,
    Untrained,
}

#[derive(Debug)]
struct Creature {
    cards: Vec<Card>,
    royal_charge: bool,
    status: CreatureStatus,
    equipment: Vec<[Card; 2]>,
}

#[derive(Debug)]
struct Player {
    library: Vec<Card>,
    graveyard: Vec<Card>,
    hand: Vec<Card>,
    life: i32,
    gems: Vec<Card>,
    creatures: Vec<Creature>,
}

impl Player {
    /// Add card, keeping the hand sorted
    fn add_card(&mut self, card: Card) {
        let card_key = |c: &Card| (
            c.color(),
            c.face,
            c.suit,
        );
        let pos = self.hand.partition_point(|c| card_key(c) < card_key(&card));
        self.hand.insert(pos, card);
    }

    /// Add gem, keeping them sorted
    fn add_gem(&mut self, gem: Card) {
        let gem_key = |c: &Card| (
            c.color(),
            c.face,
            c.suit,
        );
        let pos = self.gems.partition_point(|c| gem_key(c) < gem_key(&gem));
        self.gems.insert(pos, gem);
    }

    /// Add creature, keeping them sorted
    fn add_creature(&mut self, creature: Creature) {
        let creature_key = |c: &Creature| (
            c.cards[0].face,
            c.cards[0].color(),
            c.cards[0].suit,
        );
        let pos = self.creatures.partition_point(|c| creature_key(c) < creature_key(&creature));
        self.creatures.insert(pos, creature);
    }
}

fn random_deck<R: Rng>(player: u32, rng: &mut R) -> Vec<Card> {
    let mut deck = Vec::with_capacity(40);
    for &suit in Suit::all() {
        for &face in Face::all() {
            deck.push(Card {
                player,
                suit,
                face,
            });
        }
    }
    {
        let slice: &mut [Card] = &mut deck;
        rand::seq::SliceRandom::shuffle(slice, rng);
    }
    deck
}

fn sort_hand(hand: &mut [Card]) {
    hand.sort_by_key(|card| (
        card.color(),
        card.face,
        card.suit,
    ));
}

fn show_hand(hand: &[Card], player: u32) {
    if hand.is_empty() {
        println!("    (no cards)");
    }
    for (i, card) in hand.into_iter().enumerate() {
        println!(
            "{:>6} - {}{}",
            i + 1,
            card,
            // Show a '*' if card belongs to the other player (mind controlled)
            if card.player != player { "*" } else { "" },
        );
    }
}

fn show_gems(gems: &[Card]) {
    if gems.is_empty() {
        println!("    (no gems)");
    }
    let mut prev_color = None;
    for (i, gem) in gems.iter().enumerate() {
        let color = gem.color();
        if prev_color != Some(color) {
            prev_color = Some(color);
            println!("  {}:", color);
        } else {
            prev_color = Some(color);
        }
        println!(
            "{:>6} - {}",
            i + 1,
            gem,
        );
    }
}

fn show_creatures(creatures: &[Creature], player: u32) {
    if creatures.is_empty() {
        println!("    (no creatures)");
    }
    for (i, creature) in creatures.into_iter().enumerate() {
        print!("{:>6} - ", i + 1);
        // Show card(s)
        for (i, card) in creature.cards.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!(
                "{}{}",
                card,
                // Show a '*' if card belongs to the other player (mind
                // controlled)
                if card.player != player { "*" } else { "" },
            );
        }
        println!();
        // Show status
        match creature.status {
            CreatureStatus::Ready => {
                if creature.royal_charge {
                    println!("       Ready, Royal Charge");
                } else {
                    println!("       Ready");
                }
            }
            CreatureStatus::Tapped => println!("       Tapped"),
            CreatureStatus::Untrained => println!("       Untrained"),
        }
    }
}

enum PickedGems {
    No,
    Yes,
    Straight,
}

struct Game {
    current_player: u32,
    players: [Player; 2],
}

impl Game {
    fn play() -> Result<(), Error> {
        let mut rng = thread_rng();

        println!("New game!");

        // Pick players' decks and starting hands
        let mut decks: Vec<(Vec<Card>, Vec<Card>)> = Vec::new();
        for player in [0, 1] {
            for mulliganed in [false, true] {
                // Make a random deck
                let mut deck = random_deck(player, &mut rng);

                // Draw 5 cards
                let mut hand: Vec<Card> = deck.drain(deck.len() - 5..).collect();
                sort_hand(&mut hand);
                println!("Player {} draws cards:", player);
                show_hand(&hand, player);

                // Mulligan?
                if !mulliganed {
                    if read_yes_no("Mulligan?", Some(false))? {
                        continue;
                    }
                }

                decks.push((deck, hand));
                break;
            }
        }

        // Create players
        let mut players: Vec<Player> = Vec::new();
        for (library, hand) in decks {
            players.push(Player {
                library,
                graveyard: Vec::new(),
                hand,
                life: 20,
                gems: Vec::new(),
                creatures: Vec::new(),
            });
        }

        let mut game = Game {
            current_player: 0,
            players: players.try_into().map_err(|_| "Vec to [Player; 2]").unwrap(),
        };
        // Loop until someone wins
        game.main_loop()
    }

    fn us(&self) -> &Player {
        &self.players[self.current_player as usize]
    }

    fn us_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player as usize]
    }

    fn enemy(&self) -> &Player {
        &self.players[(1 - self.current_player) as usize]
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            println!("\nPlayer {}'s turn\n", self.current_player + 1);

            println!("Your life: {}", self.us().life);
            println!("Enemy life: {}", self.enemy().life);

            // Draw a card
            match self.us_mut().library.pop() {
                Some(card) => {
                    println!("You draw a card: {}", card);
                    self.us_mut().add_card(card);
                }
                None => println!("Can't draw, no cards left"),
            }

            let mut has_played_gem = false;

            println!("Enemy has {} cards", self.enemy().hand.len());
            println!("Enemy's gems:");
            show_gems(&self.enemy().gems);
            println!("Enemy's creatures:");
            show_creatures(&self.enemy().creatures, 1 - self.current_player);
            println!("Your creatures:");
            show_creatures(&self.us().creatures, self.current_player);
            println!("Your gems:");
            show_gems(&self.us().gems);
            println!("Your cards:");
            show_hand(&self.us().hand, self.current_player);

            while self.us().hand.len() > 0 {
                // TODO: Play gem or straight
                let card_num = read_number(
                    "Which card to play?",
                    self.us().hand.len() as i32,
                    true,
                )?;
                if card_num == 0 {
                    break;
                }
                let card = self.us_mut().hand.remove(
                    (card_num - 1) as usize,
                );
                self.play_card(card, self.current_player, &mut has_played_gem)?;
            }

            // Check victory condition
            if self.players[0].life <= 0 && self.players[1].life <= 0 {
                println!("It's a draw!");
                return Ok(());
            } else if self.players[0].life <= 0 {
                println!("Player 2 wins");
                return Ok(());
            } else if self.players[1].life <= 0 {
                println!("Player 1 wins");
                return Ok(());
            }

            self.current_player = 1 - self.current_player;
        }
    }

    fn play_card(&mut self, card: Card, player: u32, has_played_gem: &mut bool) -> Result<(), Error> {
        match card.face {
            Face::Two | Face::Three | Face::Four
            | Face::Five | Face::Six | Face::Seven => {
                if *has_played_gem {
                    println!("Can only play one gem per turn");
                } else {
                    *has_played_gem = true;
                    self.players[player as usize].add_gem(card);
                }
            }
            Face::Jack | Face::Queen | Face::King => {
                let cost = match card.face {
                    Face::Jack => 2,
                    Face::Queen => 4,
                    Face::King => 5,
                    Face::Ace => 3,
                    _ => unreachable!(),
                };
                let royal_charge = match self.pick_gems(cost, card.color())? {
                    PickedGems::No => return Ok(()),
                    PickedGems::Yes => false,
                    PickedGems::Straight => true,
                };
                let creature = Creature {
                    cards: vec![card],
                    royal_charge,
                    status: CreatureStatus::Untrained,
                    equipment: Vec::new(),
                };
                self.players[player as usize].add_creature(creature);
            }
            Face::Ace => {
                if read_yes_no("Play as spell? [y/n] ", None)? {
                    todo!();
                } else {
                    let royal_charge = match self.pick_gems(3, card.color())? {
                        PickedGems::No => return Ok(()),
                        PickedGems::Yes => false,
                        PickedGems::Straight => true,
                    };
                    let creature = Creature {
                        cards: vec![card],
                        royal_charge,
                        status: CreatureStatus::Untrained,
                        equipment: Vec::new(),
                    };
                    self.players[player as usize].add_creature(creature);
                }
            }
        }
        Ok(())
    }

    fn pick_gems(&mut self, cost: u32, color: Color) -> Result<PickedGems, Error> {
        todo!()
    }
}
