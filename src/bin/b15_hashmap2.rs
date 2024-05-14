use std::collections::HashMap;

fn main() {
    let mut stocks = HashMap::new();

    stocks.insert("Chairs", 5);
    stocks.insert("Beds", 3);
    stocks.insert("Tables", 2);
    stocks.insert("Couches", 0);

    //for (k, v) in stocks {
    //    if v == 0 {
    //        println!("{:?} out of stocks", k)
    //    } else {
    //        println!("{:?} : {:?}", k, v)
    //    }
    //}

    let mut total_stock = 0;

    for (item, qty) in stocks.iter() {
        total_stock = total_stock + qty;
        //this one replace the qty variable
        let stock_count = if *qty == 0 {
            "out of stock".to_owned()
        } else {
            //format! will be save on variable stock_count
            format!("{:?}", qty)
        };
        println!("itme={:?}, stocks={:?}", item, stock_count);
    }

    println!("total_stock = {:?}", total_stock);
}
