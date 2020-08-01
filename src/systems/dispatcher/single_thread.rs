use super::UnifiedDispatcher;
use specs::prelude::*;

pub struct SingleThreadedDispatcher<'a> {
    pub systems: Vec<Box<dyn RunNow<'a>>>
}

impl<'a> UnifiedDispatcher for SingleThreadedDispatcher<'a> {
    fn run_now(&mut self, ecs: *mut World) {
        unsafe {
            for sys in self.systems.iter_mut() {
                sys.run_now(&*ecs);
            }
        }
    }
}

macro_rules! construct_dispatcher {
    ( build [ $($inner:tt)* ] ) => {
        fn new_dispatch() -> Box<dyn UnifiedDispatcher + 'static> {
            let mut dispatch = SingleThreadedDispatcher{
                systems: Vec::new()
            };

            expand_dispatcher!(dispatch, $($inner)*);

            return Box::new(dispatch);
        }
    };
}

macro_rules! expand_dispatcher {
    ($w:expr, ) => (());
    ($dispatcher:ident , with ( $type:ident, $name:expr, $deps:expr ) $($rest:tt)*) => {
        $dispatcher.systems.push( Box::new( $type {} ));
        expand_dispatcher!($dispatcher, $($rest)*);
    };
    ($dispatcher:ident , barrier $($rest:tt)*) => {
        expand_dispatcher!($dispatcher, $($rest)*);
    };
}
