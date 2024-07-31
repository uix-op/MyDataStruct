/*
 * @Author: fang
 * @Date: 2024-07-31 10:53:01
 * @LastEditors: fang
 * @LastEditTime: 2024-07-31 14:13:03
 * @FilePath: /MyDataStruct/src/test_list.rs
 * @Description: This file is test file
 * 
 * Copyright (c) 2024 by ${uix-op}, All Rights Reserved. 
 */


#[cfg(test)]
mod test{
    use crate::data_struct::list::{NextPtr,List};
    #[test]
    fn test_insert_first(){
            let mut list = List::new();
            list.insert_to_first(1);
            let mut list_1 = List{body:NextPtr::Null};
            list_1.insert_to_first(1);
            list_1.insert_to_first(1);
            list_1.insert_to_first(1);
            match (&list.body,&list_1.body) {
                (NextPtr::Next(boxed_node_1),NextPtr::Next(boxed_node_2)) => 
                {
                    assert_eq!((*boxed_node_1).data,(*boxed_node_2).data);
                }
                _ => panic!(),
        }
    }
    #[test]
    fn test_delete_first(){
        let mut list = List::new();
        list.insert_to_first(1);
        list.insert_to_first(2);
        list.delete_from_first();
        if let NextPtr::Next(node) = &list.body{
            assert_eq!((*node).data,1);
        }
    }
    #[test]
    fn test_insert_last(){
        let mut list = List::new();
            list.insert_to_last(1);
            let mut list_1 = List{body:NextPtr::Null};
            list_1.insert_to_last(1);
            list_1.insert_to_last(1);
            list_1.insert_to_last(1);
            
            match (&list.body,&list_1.body) {
                (NextPtr::Next(boxed_node_1),NextPtr::Next(boxed_node_2)) => 
                {
                    assert_eq!((*boxed_node_1).data,(*boxed_node_2).data);
                }
                _ => panic!(),
        }
    }
}