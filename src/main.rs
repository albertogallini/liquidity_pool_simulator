
mod kfk_sw_producer; 
mod kfk_sw_consumer;
mod tokyo_pool;
use kfk_sw_producer::producer;
use kfk_sw_consumer::consumer;
use tokyo_pool::pool::{mt_liquidty_pool, tokyotest};

fn main() {

   {
        mt_liquidty_pool();
   }

   {
        tokyotest();
   }

   {
        producer::kafka_producer();
        consumer::consume();
   }
}


