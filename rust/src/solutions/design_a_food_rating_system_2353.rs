use std::{collections::{HashMap, BTreeMap, BTreeSet}, rc::Rc, cell::{RefCell, RefMut}};

struct FoodRatings {
    map: HashMap<String,Rc<RefCell<Food>>>,
    cuisine: HashMap<String,BTreeSet<Rc<RefCell<Food>>>>
}

struct Food {
    name: String,
    cuisine: String,
    rating: i32
}

impl Food {
    fn set_rating(&mut self, new_rating: i32) {
        self.rating = new_rating;
    }
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {


    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map = HashMap::with_capacity(foods.len());
        let mut cuisine_map: HashMap<String, BTreeSet<Rc<RefCell<Food>>>> = HashMap::new();
        for i in 0..foods.len() {
            let food = Food{
                name: foods[i].clone(),
                cuisine: cuisines[i].clone(),
                rating: ratings[i]
            };
            let ref1 = Rc::new(RefCell::new(food));
            let ref2 = Rc::clone(&ref1);
            food_map.insert(foods[i].clone(), ref1);
            cuisine_map.entry(cuisines[i].clone()).or_default().insert(ref2);
        }

        FoodRatings { map: food_map, cuisine: cuisine_map }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.map.get(&food).unwrap().borrow_mut().rating = new_rating;
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        panic!()
    }
}

// /**
//  * Your FoodRatings object will be instantiated and called as such:
//  * let obj = FoodRatings::new(foods, cuisines, ratings);
//  * obj.change_rating(food, newRating);
//  * let ret_2: String = obj.highest_rated(cuisine);
//  */