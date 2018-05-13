//! This module contains the code that implements meal planning
use chrono::prelude::*;
use food::*;

/// Describes a block of time
///
/// Blocks consist of a start time, an end time, and an optional food occupying the block.
pub struct Block {
    start: NaiveTime,
    end: NaiveTime,
    food: Option<Food>,
}

impl Block {
    /// Constructs a new block from its raw components
    pub fn new(start: NaiveTime, end: NaiveTime, food: Option<Food>) -> Block {
        Block {
            start: start,
            end: end,
            food: food,
        }
    }

    pub fn get_start(&self) -> &NaiveTime {
        &self.start
    }

    pub fn get_end(&self) -> &NaiveTime {
        &self.end
    }

    /// Returns true if this Block has a food attached to it, returns false otherwise
    pub fn has_food(&self) -> bool {
        self.food.is_some()
    }

    /// Returns the food attached to this Block, as an optional refrence
    pub fn get_food(&self) -> Option<&Food> {
        self.food.as_ref()
    }

    /// Adds the food to the block, overwriting the current value if it exists
    pub fn add_food(&mut self, food: Food) {
        self.food = Some(food)
    }

}
