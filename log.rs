#![macro_escape]

macro_rules! log(
    ($($arg:tt)*) => ({
        use serial::SerialFmtWriter;
        use core::fmt::FormatWriter;
        let mut w = SerialFmtWriter;
        match writeln!(w, $($arg)*) {
            _ => ()
        };
    })
)
