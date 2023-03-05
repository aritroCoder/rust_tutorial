mod pizza_order{
    pub struct Pizza{
        pub dough: String,
        pub cheeze: String,
        pub topping: String,
    }

    impl Pizza{
        pub fn lunch(topping: &str)->Pizza{
           Pizza{
                dough: String::from("regular"),
                cheeze: String::from("Mozzarella"),
                topping: String::from(topping),
           } 
        }
    }

    pub mod help_customer{
        // making help_customer public does not make child functions public. We need to declare
        // childs as public to use them outside
        fn seat_at_table(){
            println!("Customer seated at table");
        }
        //public function
        pub fn take_order(){
            seat_at_table();
            let cust_pizza: super:: Pizza = super::Pizza::lunch("Veggies"); // access Pizza struct
                                                                            // from super(parent)
                                                                            // component
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super:: Pizza){
            println!("The customer is served pizza with {}", cust_pizza.topping);
        }
    }
}

// public fn that will call the modules

pub fn ordure_food(){
   crate::restraunt::pizza_order::help_customer::take_order(); 
}
