/*
 * @Author: fang
 * @Date: 2024-07-30 18:18:53
 * @LastEditors: fang
 * @LastEditTime: 2024-07-30 21:36:19
 * @FilePath: /MyDataStruct/src/data_struct.rs
 * @Description: This file defines the data structure
 * 
 * Copyright (c) 2024 by ${uix-op}, All Rights Reserved. 
 */
pub mod list{

    pub struct List{
        pub body:NextPtr,
    }
    pub enum NextPtr {
        Null,
        Next(Box<Node>),
    }
    pub struct Node{
        pub data:i32,
        pub next:NextPtr,
    }
    impl List {
        pub fn new() -> Self{
            List { body: NextPtr::Null }
        }

        pub fn insert_to_first(&mut self,data:i32) -> (){
            if self.body == NextPtr::Null{//Determine if the body is null
                self.body = NextPtr::Next(Box::new(Node { data: data, next: NextPtr::Null }))
            }
            let _body = std::mem::replace(&mut self.body, NextPtr::Null);
            let new_node = Node{data:data,next:_body};
            self.body = NextPtr::Next(Box::new(new_node));
        }
    }
    impl PartialEq for NextPtr {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (NextPtr::Null, NextPtr::Null) => true,
                (NextPtr::Null, NextPtr::Next(_)) => false,
                (NextPtr::Next(_), NextPtr::Null) => false,
                (NextPtr::Next(data_1), NextPtr::Next(data_2)) => 
                if (*data_1).data == (*data_2).data{true}else {false},
            }
        }
    }
}

#[cfg(test)]
mod test{
    use crate::data_struct::list::NextPtr;

    use super::list::List;
    #[test]
    fn test_insert(){
        let mut list = List::new();
        list.insert_to_first(1);
        let mut list_1 = List{body:NextPtr::Null};
        list_1.insert_to_first(1);
        assert!(list.body == list_1.body);
        let values = (list.body);
    }
}