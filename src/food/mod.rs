//! This module contains dumb data structures describing real-world foods
pub mod engine;
use num_rational::*;
use std::collections::HashMap;
use chrono::Duration;

/// Describes a specific, real world food
///
/// Contains directions on how to prepare the food, as well as how much
/// of the food is made.
///
/// A food can either be a Recipe (composite of multiple foods) or
/// a RawFood (single ingredient food intended as the atomic building blocks of recipes)
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    pub fn get_name(&self) -> &IString {
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

    /// Decomposes a food into a list of ingredients
    ///
    /// TODO: Implement
    pub fn decompose(&self) -> Vec<Food> {
        Vec::new()
    }

    /// Return the time as fractional miniutes
    ///
    /// Always just returns 0 for a RawFood
    pub fn get_time(&self) -> Rational32 {
        match self {
            Food::RawFood(_) => Rational32::from_integer(0),
            Food::Recipe(x) => x.get_time()
        }
    }

        /// Time the recipe takes, in seconds
    pub fn get_duration(&self) -> Duration {
        // Get the time it takes to make the recipe, and convert it to seconds
        let time_fraction = self.get_time() * Rational32::from_integer(60);
        // We don't care about any fractions of a second left over, so we chop those off with a trunc
        let time_seconds: i64 = time_fraction.to_integer().into();

        Duration::seconds(time_seconds)
    }


}

/// Wrapper type, used to provide serde support for Rational32
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    /// Rewraps a Rational32 as a Fraction
    fn from_rational(ratio: Rational32) -> Fraction {
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
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Unit;

/// A fractional ammount combined with a unit.
///
/// Internally stored as a fraction, but preseneted as a Rational32.
/// This is to allow easy serailization/deserializeation.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Amount {
    unit: Unit,
    amount: Fraction,
}

impl Amount {
    /// Creates a new Amount given a unit and an ammount
    ///
    /// Automatically reencodes the amount as a fraction
    pub fn new(unit: Unit, amount: Rational32) -> Amount {
        Amount {
            unit: unit,
            amount: Fraction::from_rational(amount),
        }
    }

    /// Returns the unit poriton of the Ammount
    pub fn get_unit(&self) -> Unit {
        // Unit is Clone
        self.unit
    }

    /// Sets the unit poriton of the Amount
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }

    /// Returns the amount portion of the Amount
    ///
    /// Automatically unwraps the internal fraction to a Rational32
    pub fn get_amount(&self) -> Rational32 {
        self.amount.to_rational()
    }

    /// Sets the amount portion of the Amount
    ///
    /// Automatically rewraps the rational to a fraction
    pub fn set_amount(&mut self, ratio: Rational32) {
        self.amount = Fraction::from_rational(ratio);
    }
}

/// IStrings are stored as a dictonary mapping lang-code to
/// the acutal name. A shortcode (typically short, english, and hypenated)
/// is also stored for ease of refrence.
///
/// Also stores a default lang code, which is, by default, the empty string
///
/// # Valid language codes
///
/// This part of the library intentionally does not place any restrictions
/// on what can or can not be a laguage code. Language codes can be
/// any valid string, and are compared using literal equality.
///
/// # Examples
///
/// ```
/// use time_for_food::food::*;
///
/// let mut is = IString::new("hello-world");
/// is.set_default("en_US");
///
/// is.set_value_for("en_US", "Hello World!");
/// is.set_value_for("fr_FR", "Bonjour monde!");
///
/// assert_eq!(is.get_short_code(), "hello-world");
/// assert_eq!(is.get_default(), "en_US");
/// assert_eq!(is.get_value("en_US"), Some("Hello World!"));
/// assert_eq!(is.get_value("fr_FR"), Some("Bonjour monde!"));
/// assert_eq!(is.get_value("en_UK"), None);
/// ```
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IString {
    short_code: String,
    names: HashMap<String, String>,
    default: String,
}

impl IString {
    /// Constructs a new IString, given a short-code
    pub fn new(short_code: &str) -> IString {
        IString {
            short_code: short_code.to_string(),
            names: HashMap::new(),
            default: String::new(),
        }
    }

    /// Sets the value of the IString for a given lanaguge code,
    /// Creating it if it does not exist.
    pub fn set_value_for(&mut self, lang: &str, value: &str) {
        self.names.insert(lang.to_string(), value.to_string());
    }

    /// Gets the default language for this string
    pub fn get_default(&self) -> &str {
        &*self.default
    }

    /// Sets the default language for this string
    pub fn set_default(&mut self, new_default: &str) {
        self.default = new_default.to_string();
    }

    /// Gets the value of the IString for the specified language.
    ///
    /// # Returns
    /// The function will return None if it is unable to locate the
    /// desired language, and will otherwise return Some(&str) with
    /// the requested value
    pub fn get_value(&self, lang: &str) -> Option<&str> {
        let lookup = self.names.get(lang);
        if let Some(value) = lookup {
            Some(&*value)
        } else {
            None
        }
    }

    /// Returns the shortcode name for this IString
    pub fn get_short_code(&self) -> &str {
        &*self.short_code
    }
}

/// A step in making a recipe
///
/// Knows its text (encoded with an IString), and how long it takes to complete
/// (encoded as a Rational32 describing miniutes)
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step {
    text: IString,
    time: Fraction,
}

impl Step {
    /// Makes a new empty step, using a short-code name
    /// and the time to completion
    pub fn new(short_code: &str, time: Rational32) -> Step {
        Step {
            text: IString::new(short_code),
            time: Fraction::from_rational(time),
        }
    }

    /// Return an immutable refrence to the raw IString for the text
    pub fn get_text(&self) -> &IString {
        &self.text
    }

    /// Returns a mutable refrence to the raw IString
    ///
    /// For now, use this to add translations
    pub fn get_mut_text(&mut self) -> &mut IString {
        &mut self.text
    }

    /// Returns the time this step takes
    ///
    /// Units are in miniutes
    pub fn get_time(&self) -> Rational32 {
        self.time.to_rational()
    }

    /// Sets the time the step takes
    ///
    /// Units are in miniutes
    pub fn set_time(&mut self, time: Rational32) {
        self.time = Fraction::from_rational(time);
    }
}

/// Stub type, will be implemented later
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Nutrition;

/// A single ingredient, no prepration food.
/// The atomic building block of Recipes
///
/// A RawFood knows its name, its nutritonal value per serving size, its serving size,
/// as well as what unit its serving size is in.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawFood {
    /// The name of the food as a wrapped collection of strings
    name: IString,
    /// Contains the raw foods serving size as an Amount.
    /// Repusents both the Unit and the actual value
    serving_size: Amount,
    /// The nutritional value of this food
    nutrition: Nutrition,
}

impl RawFood {
    /// Constructs a new RawFood from its components
    ///
    /// While its probably not a horrible idea to call this directly,
    /// Its probably a good idea to use a RawFoodBuilder instead.
    pub fn new(name: IString, serving_size: Amount, nutrition: Nutrition) -> RawFood {
        RawFood {
            name: name,
            serving_size: serving_size,
            nutrition: nutrition,
        }
    }

    /// Returns the name (as an &IString) of this food
    pub fn get_name(&self) -> &IString {
        &self.name
    }

    /// Returns a mutable refrence to this foods name
    pub fn get_mut_name(&mut self) -> &mut IString {
        &mut self.name
    }

    /// Returns the serving size (as an amount) of this food
    pub fn get_serving_size(&self) -> Amount {
        self.serving_size
    }

    /// Returns the nutritional value of this food
    pub fn get_nutrition(&self) -> &Nutrition {
        &self.nutrition
    }
}

/// A composite Food, comprised of one or more other foods, as well as a set of
/// directions for preparing the food.
///
/// A recipe knows its name, its components foods, the ammounts required, the steps
/// required to produce the recipe, the nutritonal value of the resulting food,
/// how many servings it produces, and how long the recipe takes to make.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recipe {
    /// The name of the Recipe as an encoded String
    ///
    /// The IString wrapper type is used to help with i18n
    name: IString,
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
        name: IString,
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
            servings: Fraction::from_rational(servings),
            foods: foods,
            steps: steps,
            time: Fraction::from_rational(time),
            nutrition: nutrition,
        }
    }

    /// Returns the time the recipe takes to make, as a fractional number of miniutes
    pub fn get_time(&self) -> Rational32 {
        self.time.to_rational()
    }
}

/// Provides a builder for Recipes
///
/// As recipe is a complicated class, this provides a much more ergonomic interface.
/// Additionally provides automated wrapping of values that require it.
pub struct RecipeBuilder {
    name: IString,
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
    /// Accepts a (short code) name, and a default lanaguage,
    /// and assigns default values to all other types
    pub fn new(short_code: &str) -> RecipeBuilder {
        RecipeBuilder {
            name: IString::new(short_code),
            serving_size: None,
            servings: None,
            foods: Vec::new(),
            steps: Vec::new(),
            time: None,
            nutrition: None,
        }
    }

    /// Adds a name to the Recipe
    ///
    /// Expects a language code and a string.
    ///
    /// Will overwrite the exsiting name if given a name that already exists
    pub fn add_name(&mut self, lang_code: &str, name: &str) -> &mut Self {
        self.name.set_value_for(lang_code, name);
        self
    }

    /// Sets a serving size
    ///
    /// Takes a size and a unit, and builds an amount. Does not directly take an amount
    /// to save the consumer the step of constructing one.
    pub fn set_serving_size(&mut self, unit: Unit, amount: Rational32) -> &mut Self {
        self.serving_size = Some(Amount::new(unit, amount));
        self
    }

    /// Sets the number of servings made by the recipe
    ///
    /// Accepts a Rational32, and auto wraps it to a fraction as needed.
    /// Will overwrrite the previous value if one was set.
    pub fn set_servings(&mut self, servings: Rational32) -> &mut Self {
        self.servings = Some(servings);
        self
    }

    /// Adds an ingredient to the list of ingredients
    ///
    /// Accepts the Food, a unit, and an amount
    pub fn add_food(&mut self, food: Food, unit: Unit, amount: Rational32) -> &mut Self {
        let new_entry = (food, Amount::new(unit, amount));
        self.foods.push(new_entry);
        self
    }

    /// Adds a step to the list of steps
    ///
    /// Takes a pre-constructed step object
    pub fn add_step(&mut self, step: Step) -> &mut Self {
        self.steps.push(step);
        self
    }

    /// Sets the ammount of time that the recipe will take to make
    ///
    /// Will overwrite the existing value if one exists
    pub fn set_time(&mut self, time: Rational32) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Attaches a nutrition object to this recipe
    ///
    /// Will overwrite the existing value if one exists
    pub fn set_nutrition(&mut self, nutrition: Nutrition) -> &mut Self {
        self.nutrition = Some(nutrition);
        self
    }

    /// Creates a Recipe from the given recipe builder
    ///
    /// Will fail if any options are unset
    pub fn build_recipe(&self) -> Result<Recipe, &str> {
        let serving_size: Amount;
        let servings: Rational32;
        let time: Rational32;
        let nutrition: Nutrition;
        // Check to see if any options are unset
        if let Some(x) = self.serving_size {
            serving_size = x;
        } else {
            return Err("Serving Size not set");
        }
        if let Some(x) = self.servings {
            servings = x;
        } else {
            return Err("Servings not set");
        }
        if let Some(x) = self.time {
            time = x;
        } else {
            return Err("Time not set");
        }
        if let Some(x) = &self.nutrition {
            nutrition = x.clone();
        } else {
            return Err("Nutrition not set");
        }

        // Clone the other values
        let name = self.name.clone();
        let foods = self.foods.clone();
        let steps = self.steps.clone();

        // Construct the Recipe!
        let recipe = Recipe::new(name, serving_size, servings, foods, steps, time, nutrition);

        // Return the recipe
        Ok(recipe)
    }
}
