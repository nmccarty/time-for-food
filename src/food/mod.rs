/// This module contains dumb data structures describing real-world foods
use num_rational::*;
use serde::*;

/// Describes a specific, real world food
///
/// Contains directions on how to prepare the food, as well as how much
/// of the food is made.
///
/// A food can either be a Recipe (composite of multiple foods) or
/// a RawFood (single ingredient food intended as the atomic building blocks of recipes)
#[derive(Clone, Serialize, Deserialize)]
pub enum Food {
    Recipe(Recipe),
    RawFood(RawFood),
}

impl Food {
    /// Converts a RawFood into a Food
    pub fn new_from_raw_food(food: RawFood) -> Food {
        Food::RawFood(food)
    }
    /// Converts a Recipe into a Food
    pub fn new_from_recipe(food: Recipe) -> Food {
        Food::Recipe(food)
    }

    /// Returns the name of the Food
    pub fn get_name(&self) -> &Name {
        match *self {
            Food::RawFood(ref x) => &x.name,
            Food::Recipe(ref x) => &x.name,
        }
    }

    /// Returns the nutiritonal value of the food
    pub fn get_nutrition(&self) -> &Nutrition {
        match *self {
            Food::RawFood(ref x) => &x.nutrition,
            Food::Recipe(ref x) => &x.nutrition,
        }
    }
}

/// Wrapper type, used to provide serde support for Rational32
#[derive(Clone, Serialize, Deserialize)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    /// Rewraps a Rational32 as a Fraction
    fn from_rational(ratio: &Rational32) -> Fraction {
        Fraction {
            numerator: *ratio.numer(),
            denominator: *ratio.denom(),
        }
    }

    /// Rewraps a Fraction as a Rational32
    fn to_rational(&self) -> Rational32 {
        Rational32::new(self.numerator, self.denominator)
    }
}

/// Stub type, will be replaced by its own module later
#[derive(Clone, Serialize, Deserialize)]
pub struct Unit;

/// A fractional ammount combined with a unit.
///
/// Will be moved into the Unit modules when created.
#[derive(Clone, Serialize, Deserialize)]
pub struct Amount;

/// Stub type, will be implemented later
#[derive(Clone, Serialize, Deserialize)]
pub struct Step;

/// Stub type, will be implemented later
#[derive(Clone, Serialize, Deserialize)]
pub struct Name;

/// Stub type, will be implemented later
#[derive(Clone, Serialize, Deserialize)]
pub struct Nutrition;

/// A single ingredient, no prepration food.
/// The atomic building block of Recipes
///
/// A RawFood knows its name, its nutritonal value per serving size, its serving size,
/// as well as what unit its serving size is in.
#[derive(Clone, Serialize, Deserialize)]
pub struct RawFood {
    /// The name of the food as a wrapped collection of strings
    name: Name,
    /// Contains the raw foods serving size as an Amount.
    /// Repusents both the Unit and the actual value
    serving_size: Amount,
    /// The nutritional value of this food
    nutrition: Nutrition,
}

/// A composite Food, comprised of one or more other foods, as well as a set of
/// directions for preparing the food.
///
/// A recipe knows its name, its components foods, the ammounts required, the steps
/// required to produce the recipe, the nutritonal value of the resulting food,
/// how many servings it produces, and how long the recipe takes to make.
#[derive(Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// The name of the Recipe as an encoded String
    ///
    /// The name wrapper type is used to help with i18n
    name: Name,
    /// Contains the serving size and unit, encoded as an Amount
    serving_size: Amount,
    /// Number of servings the recipe makes.
    ///
    /// This is stored as a rational to prevent conflicts between the assumed
    /// reality and actual reality. While it would be nice to have this as in integer
    /// And force recipes to make a whole number of servings, some recipies, as input,
    /// may make a fractional number of servings.
    servings: Fraction,
    /// Contains the component foods and ammounts there of
    foods: Vec<(Food, Amount)>,
    /// Contains the steps, in order, required to produce the recipe
    steps: Vec<Step>,
    /// How long the recipe takes to make, in miniutes
    ///
    /// Stored as  Fraction rather than directly as a Rational32 to allow serde derive
    time: Fraction,
    /// Nutritional value of a serving of this Recipe
    nutrition: Nutrition,
}

impl Recipe {
    /// Constructs a new Recipe given the all the member values
    ///
    /// Probably not a good idea to use this directly,
    /// a Recipe Builder is much better idea
    pub fn new(
        name: Name,
        serving_size: Amount,
        servings: Rational32,
        foods: Vec<(Food, Amount)>,
        steps: Vec<Step>,
        time: Rational32,
        nutrition: Nutrition,
    ) -> Recipe {
        Recipe {
            name: name,
            serving_size: serving_size,
            servings: Fraction::from_rational(&servings),
            foods: foods,
            steps: steps,
            time: Fraction::from_rational(&time),
            nutrition: nutrition,
        }
    }
}

/// Provides a builder for Recipes
///
/// As recipe is a complicated class, this provides a much more ergonomic interface.
/// Additionally provides automated wrapping of values that require it.
pub struct RecipeBuilder {
    name: Name,
    serving_size: Option<Amount>,
    servings: Option<Rational32>,
    foods: Vec<(Food, Amount)>,
    steps: Vec<Step>,
    time: Option<Rational32>,
    nutrition: Option<Nutrition>,
}

impl RecipeBuilder {
    /// Creates a new RecipeBuilder
    ///
    /// Accepts a name, and assigns default values to all other types
    pub fn new(name: Name) -> RecipeBuilder {
        RecipeBuilder {
            name: name,
            serving_size: None,
            servings: None,
            foods: Vec::new(),
            steps: Vec::new(),
            time: None,
            nutrition: None,
        }
    }
}
