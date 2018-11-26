// Common format like ```2018-10-11 09:32:00```
pub static STDTIMEFMT: &'static str = "%Y-%m-%d %H:%M:%S";
// The format that bak dir uses
pub static BAKTIMEFMT: &'static str = "%Y%m%d-%H%M%S";
// ctime python Mon Jan 10 23:22:10 2018
pub static CTIMEFMT: &'static str = "%a %b %d %H:%M:%S %Y";