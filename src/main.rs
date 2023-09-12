use libloading::*;



#[link(name="raylib", kind="dylib")]
extern "C" {

}


macro_rules! reload_fns {
    ($lib: ident {$($id: ident, $name: literal : $ty: ty,)*}) => {
        $(
            let tmp: &mut Option<Symbol<$ty>> = &mut $id;
            *tmp = $lib.as_ref().unwrap().get($name).ok();
        )*
    };
}

macro_rules! reload {
    (
        $lib: ident,
        $should_close: ident,
        $should_reload: ident,
        $update: ident,
    ) => {
        unsafe{
            $update = None;
            $should_close = None;
            $should_reload = None;
            $lib = None;
            _ = $lib;
            $lib = Library::new(format!("target/debug/{}", library_filename("live").to_str().unwrap())).ok();
            reload_fns!(
                $lib{
                    $should_close, b"should_close\0": extern fn(&mut live::State)->bool,
                    $should_reload, b"should_reload\0": extern fn(&mut live::State)->bool,
                    $update, b"update\0": extern fn(&mut live::State),
                }
            );
        }
    };
}


fn main() {

    let mut lib: Option<Library>;
    let mut update;
    let mut should_close;
    let mut should_reload;

    reload!(
        lib,
        should_close,
        should_reload,
        update,
    );

    let mut g = live::graphics::Graphics::init(800, 450, "See");
    let mut state = live::init(&mut g);

    while !should_close.as_ref().unwrap()(&mut state) {
        update.as_ref().unwrap()(&mut state);

        let reload = should_reload.as_ref().unwrap()(&mut state);
        if reload {
            reload!(
                lib,
                should_close,
                should_reload,
                update,
            );
            println!("reloaded!");
        }
    }
}
