use rand::{Rng, thread_rng};
use std::io::Write;

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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
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
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    loop {
        write!(stdout, "{} [{}] ", prompt, options)?;
        stdout.flush()?;
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

fn read_number(prompt: &str, max: i32, cancellable: bool, empty: bool) -> Result<i32, Error> {
    let mut options = format!("1-{}", max);
    if cancellable {
        options.push_str(" or 0");
    }
    if empty {
        options.push_str(" or enter");
    }
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    loop {
        write!(stdout, "{} [{}] ", prompt, options)?;
        stdout.flush()?;
        match stdin.read_line(&mut buffer) {
            Ok(0) => return Err(Error::Exit),
            Err(e) => return Err(Error::Io(e)),
            Ok(_) => {
                let response = buffer.trim();
                if response == "" && empty {
                    return Ok(-1);
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

    fn next(&self) -> Option<Face> {
        match *self {
            Face::Two => Some(Face::Three),
            Face::Three => Some(Face::Four),
            Face::Four => Some(Face::Five),
            Face::Five => Some(Face::Six),
            Face::Six => Some(Face::Seven),
            Face::Seven => None,
            _ => None,
        }
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

impl Color {
    fn all() -> &'static [Color] {
        &[Color::Red, Color::Black]
    }
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
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
            Face::Ace => "A",
        };
        let suit = match self.suit {
            Suit::Spades => "\u{2660}",
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
        };
        write!(f, "{}{}", face, suit)
    }
}

#[derive(Debug)]
struct Gem {
    card: Card,
    tapped: bool,
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
    gems: Vec<Gem>,
    creatures: Vec<Creature>,
    has_played_gem: bool,
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
    fn add_gem(&mut self, card: Card) {
        let gem_key = |c: &Card| (
            c.color(),
            c.face,
            c.suit,
        );
        let pos = self.gems.partition_point(|c| gem_key(&c.card) < gem_key(&card));
        self.gems.insert(pos, Gem { card, tapped: false });
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

    fn reset(&mut self) {
        for gem in &mut self.gems {
            gem.tapped = false;
        }
        for creature in &mut self.creatures {
            creature.status = CreatureStatus::Ready;
        }
        self.has_played_gem = false;
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

fn show_gems(gems: &[Gem]) {
    if gems.is_empty() {
        println!("    (no gems)");
    }
    let mut prev_color = None;
    for (i, gem) in gems.iter().enumerate() {
        let color = gem.card.color();
        if prev_color != Some(color) {
            prev_color = Some(color);
            println!("  {}:", color);
        } else {
            prev_color = Some(color);
        }
        println!(
            "{:>6} - {}{}",
            i + 1,
            gem.card,
            if gem.tapped { " TAPPED" } else { "" },
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

fn is_straight<I: IntoIterator<Item=Face>>(card_faces: I) -> bool {
    // This relies on the fact that the hand is sorted by face
    let mut is_straight = true;
    let mut prev_face: Option<Face> = None;
    for face in card_faces {
        if let Some(prev_face) = prev_face {
            if Some(face) != prev_face.next() {
                is_straight = false;
                break;
            }
        }
        prev_face = Some(face);
    }
    is_straight
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
                println!("\nPlayer {} draws cards:", player + 1);
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
                has_played_gem: false,
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

    fn show_status(&self) {
        println!();
        println!("Your life: {}", self.us().life);
        println!("Enemy life: {}", self.enemy().life);
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
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            println!("\nPlayer {}'s turn\n", self.current_player + 1);

            // Draw a card
            match self.us_mut().library.pop() {
                Some(card) => {
                    println!("You draw a card: {}", card);
                    self.us_mut().add_card(card);
                }
                None => println!("Can't draw, no cards left"),
            }

            // Reset everything
            self.us_mut().reset();

            // Main phase
            loop {
                self.show_status();

                let untapped_gems = self.us().gems.iter().filter(|g| !g.tapped).count();
                let untapped_jacks = self.us().creatures.iter().filter(|c| c.cards[0].face == Face::Jack).count();
                let can_royal_sacrifice = Color::all().iter()
                    // For each color
                    .any(|&col| {
                        // check that we have one of each
                        let mut found = 0;
                        for creature in &self.us().creatures {
                            for card in &creature.cards {
                                if card.color() == col {
                                    found |= match card.face {
                                        Face::Jack => 0x01,
                                        Face::Queen => 0x02,
                                        Face::King => 0x04,
                                        _ => 0,
                                    };
                                }
                            }
                        }
                        found == 0x07
                    });

                if self.us().hand.len() > 0 && read_yes_no("Play a card?", Some(false))? {
                    // Play cards from our hand
                    let card_num = read_number(
                        "Which card to play?",
                        self.us().hand.len() as i32,
                        true,
                        false,
                    )?;
                    if card_num == 0 {
                        break;
                    }
                    let card = self.us_mut().hand.remove(
                        (card_num - 1) as usize,
                    );
                    match self.play_card(card, self.current_player)? {
                        None => {}
                        Some(card) => {
                            // Add card back into hand
                            self.us_mut().add_card(card);
                        }
                    }
                } else if untapped_gems > 0 && read_yes_no("Play a straight?", Some(false))? {
                    // Play a straight from our gems
                    todo!();
                } else if untapped_gems > 0 && read_yes_no("Sacrifice a gem?", Some(false))? {
                    // Sacrifice one of our gems
                    todo!();
                } else if untapped_jacks >= 2 && read_yes_no("Stack jacks?", Some(false))? {
                    // Stack untapped jacks from our creatures
                    todo!();
                } else if can_royal_sacrifice && read_yes_no("Royal sacrifice?", Some(false))? {
                    // Royal sacrifice
                    todo!();
                } else {
                    break;
                }
            }

            // TODO: Attack, defense

            // TODO: Second main phase

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

            // Next turn
            self.current_player = 1 - self.current_player;
        }
    }

    /// Try to play a card, asking relevant questions.
    ///
    /// Returns Ok(None) if the card was played, or Ok(Some(card)) if the card
    /// couldn't be played and should be returned to the player's hand.
    fn play_card(&mut self, card: Card, player: u32) -> Result<Option<Card>, Error> {
        match card.face {
            Face::Two | Face::Three | Face::Four
            | Face::Five | Face::Six | Face::Seven => {
                if self.players[player as usize].has_played_gem {
                    println!("You can only play one gem per turn");
                    Ok(Some(card))
                } else {
                    println!("Adding gem {}", card);
                    self.players[player as usize].has_played_gem = true;
                    self.players[player as usize].add_gem(card);
                    Ok(None)
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
                let royal_charge = match self.pick_gems(cost, player, card.color())? {
                    PickedGems::No => return Ok(Some(card)),
                    PickedGems::Yes => false,
                    PickedGems::Straight => true,
                };
                println!("Adding creature {} (untrained)", card);
                if royal_charge {
                    println!("Creature has Royal Charge!");
                }
                let creature = Creature {
                    cards: vec![card],
                    royal_charge,
                    status: CreatureStatus::Untrained,
                    equipment: Vec::new(),
                };
                self.players[player as usize].add_creature(creature);
                Ok(None)
            }
            Face::Ace => {
                if read_yes_no("Play as spell? [y/n] ", None)? {
                    todo!();
                } else {
                    let royal_charge = match self.pick_gems(3, player, card.color())? {
                        PickedGems::No => return Ok(Some(card)),
                        PickedGems::Yes => false,
                        PickedGems::Straight => true,
                    };
                    println!("Adding creature {} (trained)", card);
                    if royal_charge {
                        println!("Creature has Royal Charge!");
                    }
                    let creature = Creature {
                        cards: vec![card],
                        royal_charge,
                        status: CreatureStatus::Untrained,
                        equipment: Vec::new(),
                    };
                    self.players[player as usize].add_creature(creature);
                    Ok(None)
                }
            }
        }
    }

    fn pick_gems(&mut self, cost: u32, player: u32, color: Color) -> Result<PickedGems, Error> {
        // TODO: Gem sacrifice 4 or 5 to "mine" (+3 energy)

        // Create vector of same size as gems
        // For gems we can't select: None
        // For gems we can select Some(true) if selected, Some(false) if unselected
        let mut choices = Vec::new();
        let mut num_choices = 0;
        for gem in &self.players[player as usize].gems {
            if !gem.tapped && gem.card.color() == color {
                choices.push(Some(false));
                num_choices += 1;
            } else {
                choices.push(None);
            }
        }
        let mut num_selected = 0;

        // Loop until gems are selected and confirmed
        loop {
            for (i, (gem, selected)) in
                self.players[player as usize].gems.iter().zip(&choices)
                    .filter_map(|(gem, &choice)|
                        match choice { Some(c) => Some((gem, c)), None => None }
                    )
                    .enumerate()
            {
                println!(
                    "{:>6} - {} {}",
                    i + 1,
                    if selected { "[x]" } else { "[ ]" },
                    gem.card,
                );
            }
            match read_number(
                &format!("Pick gems ({}/{})", num_selected, cost),
                num_choices,
                true,
                num_selected == cost,
            )? {
                // Cancelled
                0 => return Ok(PickedGems::No),
                // Confirmed
                -1 => {
                    // Tap selected gems
                    for (gem, &selected) in self.players[player as usize].gems.iter_mut().zip(&choices) {
                        if let Some(true) = selected {
                            gem.tapped = true;
                        }
                    }

                    // Check for straights
                    // This relies on the fact that the hand is sorted by face
                    let is_straight = is_straight(
                        self.players[player as usize].gems.iter().zip(&choices)
                            .filter_map(|(gem, &selected)|
                                if let Some(true) = selected {
                                    Some(gem.card.face)
                                } else {
                                    None
                                }
                            )
                    );

                    if is_straight {
                        return Ok(PickedGems::Straight);
                    } else {
                        return Ok(PickedGems::Yes);
                    }
                }
                // Toggle a gem
                i => {
                    match choices.iter_mut()
                        .filter_map(|s| s.as_mut())
                        .nth((i - 1) as usize)
                    {
                        Some(selected) => match *selected {
                            true => {
                                *selected = false;
                                num_selected -= 1;
                            }
                            false => {
                                *selected = true;
                                num_selected += 1;
                            }
                        }
                        None => panic!(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Face, is_straight};

    #[test]
    fn test_is_straight() {
        assert_eq!(is_straight([Face::Two]), true);
        assert_eq!(is_straight([Face::Two, Face::Three]), true);
        assert_eq!(is_straight([Face::Two, Face::Two]), false);
        assert_eq!(is_straight([Face::Two, Face::Four]), false);
        assert_eq!(is_straight([Face::Three, Face::Four, Face::Five]), true);
        assert_eq!(is_straight([Face::Three, Face::Four, Face::Six]), false);
        assert_eq!(is_straight([Face::Three, Face::Four, Face::Four]), false);
        assert_eq!(
            is_straight([Face::Three, Face::Four, Face::Five, Face::Six]),
            true,
        );
        assert_eq!(
            is_straight([Face::Three, Face::Four, Face::Five, Face::Seven]),
            false,
        );
        assert_eq!(
            is_straight([Face::Two, Face::Four, Face::Five, Face::Six]),
            false,
        );
        assert_eq!(
            is_straight([Face::Three, Face::Four, Face::Four, Face::Five]),
            false,
        );
        assert_eq!(
            is_straight([Face::Three, Face::Four, Face::Four, Face::Six]),
            false,
        );
    }
}
