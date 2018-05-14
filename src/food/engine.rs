//! This module contains the code that implements meal planning
use chrono::prelude::*;
use food::*;

/// Describes a block of time
///
/// Blocks consist of a start time, an end time, and an optional food occupying the block.
///
/// NaiveTime is used as our goal is to produce a timezone agnostic schedule.
pub struct Block {
    start: NaiveTime,
    end: NaiveTime,
    food: Option<Food>,
}

impl Block {
    /// Constructs a new block from its raw components
    pub fn new(start: NaiveTime, end: NaiveTime, food: Option<Food>) -> Block {
        Block {
            start,
            end,
            food,
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

    /// Attempts to split the block into two blocks, placing the food at the start
    ///
    /// # Returns
    ///
    /// If the block is able to be split by the food, this function will return the new
    /// pair of blocks. Otherwise, it will return a NaiveTime containing the end time
    /// this block would need to have in order to be split by this food.
    ///
    /// The second block is optional, as there will not be a second block if there is no existing
    /// food and the amount of time avaible is exactly consumed by the given food.
    ///
    /// Will place the existing food in the second block, if it exists.
    pub fn split_at_start(&self, food: &Food) -> Result<(Block, Option<Block>), NaiveTime> {
        // First, calcuate the end time that would result from making this food
        let food_end: NaiveTime;
        if let Some(ref existing_food) = self.food {
            // Food exists, work that into our calcuations
            food_end = self.start + existing_food.get_duration() + food.get_duration();
        } else {
            // Food does not exist, proceed as normal
            food_end = self.start + food.get_duration();
        }

        // Check to see if the food will fit
        if food_end > self.end {
            // Food does not fit, report our failure
            Err(food_end)
        } else {
            // Capture a copy of the existing food to put into the new second block
            let existing_food = self.food.clone();
            // Capture a copy of the new food to put into the new first block
            let new_food = food.clone();

            // Calcuate the split point.
            // Will *NOT* be the same as food_end when there is an existing food
            let middle = self.start + food.get_duration();

            // Handle the case where the avaible time is exactly consumed, and there
            // is no existing food
            if middle == self.end && existing_food.is_some() {
                // In this case, we basically just create a copy of the block
                Ok((Block::new(self.start, self.end, Some(new_food)), None))
            } else {
                let first_block = Block::new(self.start, middle, Some(new_food));
                let second_block = Block::new(middle, self.end, existing_food);
                Ok((first_block, Some(second_block)))
            }
        }
    }
}
