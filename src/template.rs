use rand::seq::SliceRandom;
use rand::{Rng, seq::IndexedRandom};
use std::collections::HashMap;
use tera::{Context, Tera, Value};

pub fn render_prompt(prompt_template: &str) -> String {
    let mut tera = Tera::default();

    // Register the custom function
    tera.register_function("random_color", random_color);
    tera.register_function("random_animal", random_animal);
    tera.register_function("random_vegetable", random_vegetable);
    tera.register_function("random_fruit", random_fruit);

    let context = Context::new(); // No need for variables in this simple example

    match tera.render_str(prompt_template, &context) {
        Ok(rendered) => rendered,
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            // Panic on error as per custom instructions
            prompt_template.to_string()
        }
    }
}

pub fn random_color(_args: &HashMap<String, Value>) -> tera::Result<tera::Value> {
    let mut rng = rand::rng();

    let random_element = *COLORS.choose(&mut rng).unwrap();

    Ok(Value::from(random_element))
}

pub fn random_animal(_args: &HashMap<String, Value>) -> tera::Result<tera::Value> {
    let mut rng = rand::rng();

    let random_element = *ANIMALS.choose(&mut rng).unwrap();

    Ok(Value::from(random_element))
}

pub fn random_vegetable(_args: &HashMap<String, Value>) -> tera::Result<tera::Value> {
    let mut rng = rand::rng();

    let random_element = *VEGETABLES.choose(&mut rng).unwrap();

    Ok(Value::from(random_element))
}

pub fn random_fruit(_args: &HashMap<String, Value>) -> tera::Result<tera::Value> {
    let mut rng = rand::rng();

    let random_element = *FRUITS.choose(&mut rng).unwrap();

    Ok(Value::from(random_element))
}

pub fn random_fruit_or_vegetable(_args: &HashMap<String, Value>) -> tera::Result<tera::Value> {
    let mut rng = rand::rng();

    let elements = [FRUITS, VEGETABLES].concat();

    let random_element = *elements.choose(&mut rng).unwrap();

    Ok(Value::from(random_element))
}

const FRUITS: [&str; 25] = [
    "apple",
    "banana",
    "cherry",
    "date",
    "grape",
    "kiwi",
    "lemon",
    "mango",
    "orange",
    "peach",
    "pear",
    "plum",
    "pineapple",
    "raspberry",
    "strawberry",
    "watermelon",
    "blueberry",
    "blackberry",
    "grapefruit",
    "tangerine",
    "papaya",
    "melon",
    "apricot",
    "coconut",
    "lime",
];

const VEGETABLES: [&str; 25] = [
    "carrot",
    "broccoli",
    "cauliflower",
    "pea",
    "green bean",
    "lettuce",
    "tomato",
    "potato",
    "onion",
    "bell pepper",
    "cabbage",
    "spinach",
    "kale",
    "zucchini",
    "cucumber",
    "corn",
    "asparagus",
    "celery",
    "eggplant",
    "garlic",
    "mushroom",
    "radish",
    "turnip",
    "beet",
    "brussels sprout",
];

const ANIMALS: [&str; 155] = [
    "Albatross",
    "Alligator",
    "Anteater",
    "Antelope",
    "Armadillo",
    "Badger",
    "Bat",
    "Bear",
    "Beaver",
    "Bison",
    "Boar",
    "Buffalo",
    "Camel",
    "Cat",
    "Centipede",
    "Chameleon",
    "Cheetah",
    "Chicken",
    "Chimpanzee",
    "Clam",
    "Cormorant",
    "Cow",
    "Crab",
    "Crane",
    "Crocodile",
    "Crow",
    "Deer",
    "Dingo",
    "Dog",
    "Dolphin",
    "Donkey",
    "Dove",
    "Duck",
    "Eagle",
    "Echidna",
    "Eel",
    "Elephant",
    "Emu",
    "Falcon",
    "Ferret",
    "Fish",
    "Flamingo",
    "Fox",
    "Frog",
    "Gazelle",
    "Gecko",
    "Gerbil",
    "Giraffe",
    "Gnu",
    "Goat",
    "Goose",
    "Guinea pig",
    "Hamster",
    "Hare",
    "Hawk",
    "Hedgehog",
    "Heron",
    "Hippopotamus",
    "Horse",
    "Hummingbird",
    "Hyena",
    "Iguana",
    "Impala",
    "Jaguar",
    "Jaguarundi",
    "Jellyfish",
    "Kangaroo",
    "Kitten",
    "Kiwi",
    "Koala",
    "Komodo Dragon",
    "Lemur",
    "Leopard",
    "Lion",
    "Lizard",
    "Llama",
    "Lobster",
    "Lynx",
    "Marmoset",
    "Marten",
    "Meerkat",
    "Millipede",
    "Mink",
    "Mole",
    "Monkey",
    "Moose",
    "Mouse",
    "Narwhal",
    "Newt",
    "Octopus",
    "Ostrich",
    "Otter",
    "Owl",
    "Oyster",
    "Panda",
    "Panther",
    "Parrot",
    "Peacock",
    "Pelican",
    "Penguin",
    "Pig",
    "Pigeon",
    "Platypus",
    "Porcupine",
    "Puffin",
    "Quail",
    "Quetzal",
    "Rabbit",
    "Raccoon",
    "Rat",
    "Raven",
    "Rhea",
    "Rhinoceros",
    "Salamander",
    "Scallop",
    "Scorpion",
    "Seagull",
    "Seal",
    "Sea Urchin",
    "Shark",
    "Sheep",
    "Shrimp",
    "Skunk",
    "Sloth",
    "Snail",
    "Snake",
    "Sparrow",
    "Spider",
    "Squid",
    "Squirrel",
    "Starfish",
    "Stork",
    "Swan",
    "Tapir",
    "Tarantula",
    "Tasmanian Devil",
    "Tiger",
    "Toad",
    "Tortoise",
    "Toucan",
    "Turkey",
    "Turtle",
    "Urchin",
    "Vicuna",
    "Vulture",
    "Wallaby",
    "Walrus",
    "Weasel",
    "Whale",
    "Wolf",
    "Wombat",
    "Woodpecker",
    "Yak",
    "Zebra",
    "Zebu",
];

const COLORS: [&str; 66] = [
    "mauve",
    "sepia",
    "azurro",
    "fuchsia",
    "cobalt blue",
    "cadmium red",
    "cerulean blue",
    "viridian",
    "alizarin crimson",
    "burnt sienna",
    "yellow ochre",
    "ultramarine",
    "payne's grey",
    "turquoise",
    "magenta",
    "prussian blue",
    "indigo",
    "phthalo green",
    "quinacridone gold",
    "raw umber",
    "titanium white",
    "ivory black",
    "mars black",
    "naples yellow",
    "cadmium yellow",
    "sap green",
    "burnt umber",
    "raw sienna",
    "dioxazine purple",
    "hooker's green",
    "cadmium orange",
    "vermilion",
    "permanent rose",
    "permanent green",
    "carmine",
    "cobalt teal",
    "zinc white",
    "buff titanium",
    "brilliant green",
    "caput mortuum",
    "neutral tint",
    "quinacridone magenta",
    "indian red",
    "hansa yellow",
    "pyrrole red",
    "cerulean blue hue",
    "light red",
    "terracotta",
    "violet iron oxide",
    "gold ochre",
    "cobalt violet",
    "smalt",
    "perylene green",
    "iridescent bronze",
    "cobalt turquoise",
    "undersea green",
    "mineral violet",
    "chrome oxide green",
    "perylene red",
    "quinacridone burnt orange",
    "perylene maroon",
    "green gold",
    "nickel azo yellow",
    "van dyke brown",
    "rose madder genuine",
    "paris blue",
];
