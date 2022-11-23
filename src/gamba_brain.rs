use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;

pub(crate) fn gamba_brain(
    input_character: &str,
    roll_amount: u32,
    mut pity_4star: u8,
    mut pity_5star: u32,
) {
    let mut count = 0;
    let mut character_acquired = false;
    let rarities = [3, 4, 5];
    let mut weights = [0.943, 0.051, 0.006];
    let mut rng = thread_rng();
    let four_star_pool = ["Thoma", "Shikanoin Heizou", "Layla"];
    let mut four_star_guarantee = false;
    let mut obtained_character: &str = "";
    let five_star_pool = ["Yae Miko"];
    let five_star_50 = ["Keqing", "Tighnari", "Mona", "Jean", "Diluc", "Qiqi"];
    let mut five_star_guarantee = false;
    //let roll_amount = 70;
    //let mut pity_4star = 9;
    //let mut pity_5star = 4;
    let mut fate: usize;
    let desired_character = input_character.trim();
    let mut character_present_in_4star: bool = false;
    let mut character_present_in_5star: bool = false;

    /* Lord forgive me for what I'm about to code */

    for character in four_star_pool {
        if character == desired_character {
            character_present_in_4star = true;
        }
    }
    for character in five_star_pool {
        if character == desired_character {
            character_present_in_5star = true;
        }
    }
    for character in five_star_50 {
        if character == desired_character {
            character_present_in_5star = true;
            //println!("gud :D {}, {}", character_present_in_5star, character_present_in_4star);
        }
    }

    if (character_present_in_4star == true) || (character_present_in_5star == true) {
        while character_acquired == false {
            count += 1;
            if pity_5star >= 74 {
                weights[2] = 0.25;
            } else {
                weights[2] = 0.006;
            }
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rarity = rarities[dist.sample(&mut rng)];
            //checking if pity is high enough for guaranteed
            if pity_4star == 9 {
                rarity = 4;
                pity_4star = 0;
            }
            if pity_5star == 89 {
                rarity = 5;
                pity_5star = 0;
            }
            if rarity == 3 {
                pity_4star += 1;
                pity_5star += 1;
                obtained_character = "3_star";
                println!("{}-star, random weapon, roll number {}", rarity, count);
            }
            //handling a 4-star roll
            if rarity == 4 {
                pity_4star = 0;
                pity_5star += 1;
                //checks if guarantee exists
                //we generate a random number from one to three, each number corresponds to a certain character
                //0 = 4star-1, 1 = 4star-2, 2 = 4star-3, 3 = lost the 50/50 - here the user would get something from the generic 4-star pool but i can't be bothered to look that up
                if four_star_guarantee == true {
                    fate = rand::thread_rng().gen_range(0..=2);
                } else {
                    fate = rand::thread_rng().gen_range(0..=3);
                }
                if fate == 3 {
                    four_star_guarantee = true;
                    println!("{}-star, You lost the 50/50! Roll number {}", rarity, count)
                } else {
                    obtained_character = four_star_pool[fate];
                    if four_star_guarantee == true {
                        four_star_guarantee = false;
                        println!(
                            "{}-star, You obtained guaranteed {}! Roll number {}",
                            rarity, obtained_character, count
                        );
                    } else {
                        println!(
                            "{}-star, You obtained {}! Roll number {}",
                            rarity, obtained_character, count
                        );
                    }
                }
            }
            //handling a 5-star roll
            if rarity == 5 {
                pity_4star += 1;
                pity_5star = 0;
                //same as in 4-star but there's less things
                //0 = featured 5 star, 1 = lost 50/50
                if five_star_guarantee == true {
                    fate = 0;
                } else {
                    fate = rand::thread_rng().gen_range(0..=1);
                }
                if fate == 1 {
                    five_star_guarantee = true;
                    fate = rand::thread_rng().gen_range(0..=5);
                    obtained_character = five_star_50[fate];
                    println!(
                        "{}-star, You lost the 50/50! You obtained {}! Roll number {}",
                        rarity, obtained_character, count
                    );
                } else {
                    //need to have two to check if the featured 5-star was guaranteed or from a won 50/50
                    if five_star_guarantee == true {
                        five_star_guarantee = false;
                        obtained_character = five_star_pool[fate];
                        println!(
                            "{}-star, You obtained guaranteed {}! Roll number {}",
                            rarity, obtained_character, count
                        );
                    } else {
                        obtained_character = five_star_pool[fate];
                        println!(
                            "{}-star, You obtained {}! Roll number {}",
                            rarity, obtained_character, count
                        );
                    }
                }
            }

            if obtained_character == desired_character {
                character_acquired = true;
                println!(
                    "{} acquired! You only needed {} rolls 💀",
                    desired_character, count
                );
            }

            /*println!("{}*, roll number {}", rarity, count);*/
            if count == roll_amount {
                if character_acquired == false {
                    println!("🗿 YOU DIDN'T GET {} RIP XD", desired_character)
                }
                break;
            }
        }
    } else {
        println!(
            "The character {} isn't in the character list 🗿",
            desired_character
        );
    }
}
