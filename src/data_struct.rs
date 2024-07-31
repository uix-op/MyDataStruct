/*
 * @Author: fang
 * @Date: 2024-07-30 18:18:53
 * @LastEditors: fang
 * @LastEditTime: 2024-07-31 14:21:38
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
            self.body = NextPtr::Next(Box::new(Node{
                data:data,
                next:std::mem::replace(&mut self.body, NextPtr::Null),
            }));
        }
        pub fn delete_from_first(&mut self) -> (){
            if let NextPtr::Next(boxed_node) =
             std::mem::replace(&mut self.body, NextPtr::Null){
                self.body = (*boxed_node).next;
            }
        }
        pub fn insert_to_last(&mut self,data:i32) -> (){
            if self.body == NextPtr::Null{
                self.body = NextPtr::Next(Box::new(Node{data:data,next:NextPtr::Null}));
            }
            let mut link_next = &mut self.body;
            while let NextPtr::Next(boxed_node) = link_next {
                let _link_next = &mut (*boxed_node).next;
                link_next = _link_next;
            }
            if let NextPtr::Next(ref mut next_node) = link_next{
                (*next_node).next = NextPtr::Next(Box::new(Node { data, next: NextPtr::Null }));
            }
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
    //about drop func
    impl Drop for List {
        fn drop(&mut self) {
            let mut cur_link = std::mem::replace(&mut self.body, NextPtr::Null);
            while let NextPtr::Next(mut boxed_node) = cur_link {
                cur_link = std::mem::replace(&mut boxed_node.next, NextPtr::Null);
                //Tail recursion
            }
        }
    }
}
