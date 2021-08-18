error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Window(::sdl2::video::WindowBuildError);
        Sdl(::sdl2::IntegerOrSdlError);
        Image(::imagesize::ImageError);
    }
}
