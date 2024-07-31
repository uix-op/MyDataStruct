/*
 * @Author: fang
 * @Date: 2024-07-31 10:53:01
 * @LastEditors: fang
 * @LastEditTime: 2024-07-31 11:10:42
 * @FilePath: /MyDataStruct/src/test_list.rs
 * @Description: This file is test file
 * 
 * Copyright (c) 2024 by ${uix-op}, All Rights Reserved. 
 */


#[cfg(test)]
mod test{
    use crate::data_struct::list::{NextPtr,List};
    #[test]
    fn test_insert(){
            let mut list = List::new();
            list.insert_to_first(1);
            let mut list_1 = List{body:NextPtr::Null};
            list_1.insert_to_first(1);
            list_1.insert_to_first(1);
            list_1.insert_to_first(1);
            match (list.body,list_1.body) {
                (NextPtr::Next(a),NextPtr::Next(b)) => {
                    assert!((*a).data == (*b).data);
                }
            _ => panic!()
        }
    }
    fn test_delete(){
        
    }
}