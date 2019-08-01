pub mod hosting;

mod serving{
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
    fn something(){
        super::hosting::add_to_waitlist();
    }
}