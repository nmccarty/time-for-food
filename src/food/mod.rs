/// This module contains dumb data structures describing real-world foods

/// Describes a specific, real world food
///
/// Contains directions on how to prepare the food, as well as how much
/// of the food is made.
///
/// A food can either be a Recipe (composite of multiple foods) or
/// a RawFood (single ingredient food intended as the atomic building blocks of recipes)
pub enum Food {
    Recipe(Recipe),
    RawFood(RawFood),
}

/// Stub type, will be replaced by its own module later
pub struct Unit;

/// A fractional ammount combined with a unit.
///
/// Will be moved into the Unit modules when created.
pub struct Amount;

/// Stub type, will be implemented later
pub struct Step;

/// Stub type, will be implemented later
pub struct Name;

/// Stub type, will be implemented later
pub struct Nutrition;

/// A single ingredient, no prepration food.
/// The atomic building block of Recipes
///
/// A RawFood knows its name, its nutritonal value per serving size, its serving size,
/// as well as what unit its serving size is in.
pub struct RawFood {
    /// The name of the food as a wrapped collection of strings
    name: Name,
    /// Contains the raw foods serving size as an Amount.
    /// Repusents both the Unit and the actual value
    serving_size: Amount,
}

/// A composite Food, comprised of one or more other foods, as well as a set of
/// directions for preparing the food.
///
/// A recipe knows its name, its components foods, the ammounts required, the steps
/// required to produce the recipe, the nutritonal value of the resulting food,
/// how many servings it produces, and how long the recipe takes to make.
pub struct Recipe;

