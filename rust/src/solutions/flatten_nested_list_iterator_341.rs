use std::mem::discriminant;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
    stack: Vec<NestedInteger>
}


impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        Self {
            stack: vec![NestedInteger::List(nestedList)]
        }
    }

    fn cleanup(&mut self) {
        while
            !self.stack.is_empty() &&
            match self.stack.last().unwrap() {
                NestedInteger::List(_) => true,
                _ => false
            }
        {
            match self.stack.pop().unwrap() {
                NestedInteger::List(list) => {
                    for nested in list.into_iter().rev() {
                        self.stack.push(nested);
                    }
                },
                _ => panic!("wtf")
            }
        }
    }
    
    fn next(&mut self) -> i32 {
        match self.stack.pop().unwrap() {
            NestedInteger::List(_) => panic!("wtf"),
            NestedInteger::Int(num) => num
        }
    }
    
    fn has_next(&mut self) -> bool {
        self.cleanup();
        !self.stack.is_empty()
    }
}