use specs::prelude::*;
use super::UnifiedDispatcher;

pub struct MultiThreadedDispatcher {
    pub dispatcher: specs::Dispatcher<'static, 'static>
}

impl<'a> UnifiedDispatcher for MultiThreadedDispatcher {
    fn run_now(&mut self, ecs: *mut World) {
        unsafe {
            self.dispatcher.dispatch(&mut *ecs);
        }
    }
}

macro_rules! construct_dispatcher {
    ( build [ $($inner:tt)* ] ) => {
        fn new_dispatch() -> Box<dyn UnifiedDispatcher + 'static> {
            use specs::DispatcherBuilder;

            let mut dispatcher = DispatcherBuilder::new();
            expand_dispatcher!(dispatcher, $($inner)*);

            let dispatch = MultiThreadedDispatcher{
                dispatcher: dispatcher.build()
            };

            return Box::new(dispatch);
        }
    };
}

macro_rules! expand_dispatcher {
    ($w:expr, ) => (());
    ($dispatcher:ident , with ( $type:ident, $name:expr, $deps:expr ) $($rest:tt)*) => {
        $dispatcher = $dispatcher.with($type{}, $name, $deps);
        expand_dispatcher!($dispatcher, $($rest)*);
    };
    ($dispatcher:ident , barrier $($rest:tt)*) => {
        $dispatcher = $dispatcher.with_barrier();
        expand_dispatcher!($dispatcher, $($rest)*);
    };
}
